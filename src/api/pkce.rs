use std::time::Duration;

use axum::body::Body;
use axum::response::{IntoResponse, Response};
use base64::prelude::*;
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use sha2::Digest;
use tokio::sync::mpsc;
use tokio::time::timeout;

use crate::AppState;
use crate::db::{User, UserId};
use crate::error::AppError;
use crate::traits::RequestBody;

/// How long a PKCE authentication request is valid for.
const PKCE_TIMEOUT: Duration = Duration::from_secs(20 * 60); // 20 minutes

/// How long to wait for a PKCE long poll request before requiring the client to
/// send a new request.
///
/// See https://datatracker.ietf.org/doc/html/rfc6202#section-5.5
const PKCE_LONG_POLL_TIMEOUT: Duration = Duration::from_secs(30); // 30 seconds

/// Proof key code exchange for securely signing into desktop apps.
///
/// This is implemented based on
/// https://developer.okta.com/blog/2018/12/13/oauth-2-for-native-and-mobile-apps.
///
/// 1. The user clicks "sign in" in a desktop app such as Hyperspeedcube.
/// 2. The desktop app generates a secret value `v`.
/// 3. The desktop app opens a link to the leaderboards website in a web browser
///    that contains a hash of `v` in the URL parameter.
/// 4. The leaderboards website prompts for sign-in, if the user is not already
///    signed in. This follows the normal sign-in flow.
/// 5. The leaderboards website prompts the user whether they want to authorize
///    the desktop app.
/// 6. If the user confirms, then the leaderboards website sends a request
///    containing the user's token and the hash of `v`.
#[derive(Debug)]
pub struct PkceHash {
    /// Time when the request expires.
    pub expiry: DateTime<Utc>,
    /// Output channel or user ID.
    pub output: PkceHashOutput,
}

#[derive(Debug)]
pub enum PkceHashOutput {
    /// Nothing has happened.
    Idle,
    /// Desktop app is polling. Send user ID on the channel when the user has
    /// authenticated.
    LongPollChannel(mpsc::Sender<UserId>),
    /// User has authenticated. Awaiting poll from desktop app.
    Authenticated(UserId),
}
impl PkceHash {
    pub fn new() -> Self {
        let expiry = Utc::now() + PKCE_TIMEOUT;

        Self {
            expiry,
            output: PkceHashOutput::Idle,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expiry
    }
}

#[derive(serde::Deserialize)]
pub struct LongPollPkceRequest {
    secret_code: String,
}
impl RequestBody for LongPollPkceRequest {
    type Response = Response<Body>;

    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let hash = BASE64_URL_SAFE.encode(sha2::Sha256::digest(self.secret_code));
        let mut pkces = state.pkce_hash_values.lock().await;
        let pkce = pkces.entry(hash).or_insert_with(PkceHash::new);

        if pkce.is_expired() {
            return Err(AppError::AuthenticationTimeout);
        }

        let user_id = match &pkce.output {
            // if there is an old channel, remove it and get a new one
            PkceHashOutput::Idle | PkceHashOutput::LongPollChannel(_) => {
                let (tx, mut rx) = mpsc::channel(1);
                pkce.output = PkceHashOutput::LongPollChannel(tx);
                drop(pkces); // unlock mutex
                match timeout(PKCE_LONG_POLL_TIMEOUT, rx.recv()).await {
                    Ok(Some(user_id)) => Some(user_id), // success!
                    Ok(None) => None,                   // channel dropped (replaced)
                    Err(_) => None,                     // timeout
                }
            }

            PkceHashOutput::Authenticated(user_id) => Some(*user_id),
        };

        Ok(match user_id {
            Some(user_id) => (
                StatusCode::OK,
                Body::from(state.create_token(user_id).await?.string),
            ),

            // client should send another request
            None => (StatusCode::NO_CONTENT, Body::empty()),
        }
        .into_response())
    }
}
