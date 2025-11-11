use axum::response::Response;
use axum_typed_multipart::TryFromMultipart;

use crate::api::pkce::{PkceHash, PkceHashOutput};
use crate::db::User;
use crate::{AppError, AppState, RequestBody};

#[derive(serde::Deserialize)]
pub struct ConfirmPkcePage {
    #[allow(unused)] // used by JS to fill in form
    hash: String,
}

impl RequestBody for ConfirmPkcePage {
    type Response = Response;

    async fn request(
        self,
        _state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        if user.is_none() {
            return Err(AppError::NotLoggedIn);
        }

        Ok(crate::render_html_template(
            "submit-pkce.html",
            &user,
            serde_json::json!({}),
        ))
    }
}

#[derive(TryFromMultipart)]
pub struct ConfirmPkceRequest {
    hash: String,
}
impl RequestBody for ConfirmPkceRequest {
    type Response = Response;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        // Cleans very old requests, but keep slightly-expired ones
        state.clean_pkce_requests().await;

        let user = user.ok_or(AppError::NotLoggedIn)?;

        tracing::trace!(user = ?user.id, hash = ?self.hash, "Authenticating PKCE");
        let mut pkces = state.pkce_hash_values.lock().await;
        let pkce = pkces.entry(self.hash.clone()).or_insert_with(PkceHash::new);

        if pkce.is_expired() {
            return Err(AppError::AuthenticationTimeout);
        }

        match &pkce.output {
            PkceHashOutput::Idle => pkce.output = PkceHashOutput::Authenticated(user.id),
            PkceHashOutput::LongPollChannel(sender) => {
                let _ = sender.send(user.id).await; // don't care if error
                pkce.output = PkceHashOutput::Authenticated(user.id); // in case the old request timed out
            }
            PkceHashOutput::Authenticated(user_id) => {
                tracing::warn!("Authenticated PKCE {} twice: once for {user_id}", self.hash);
                return Err(AppError::NotAuthorized);
            }
        }

        Ok(crate::render_html_template(
            "confirmed-pkce.html",
            &Some(user),
            serde_json::json!({}),
        ))
    }
}
