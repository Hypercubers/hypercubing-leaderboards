use std::fmt;
use std::time::Duration;

use axum::response::{AppendHeaders, IntoResponse, Redirect, Response};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use chrono::{DateTime, TimeDelta, Utc};
use reqwest::header::SET_COOKIE;

use crate::db::{User, UserId};
use crate::traits::Linkable;
use crate::{AppError, AppResult, AppState};

const EXPIRED_TOKEN: &str = "token=expired; Expires=Thu, 1 Jan 1970 00:00:00 GMT";
pub const APPEND_EXPIRED_TOKEN: AppendHeaders<Option<(axum::http::header::HeaderName, &str)>> =
    AppendHeaders(Some((SET_COOKIE, EXPIRED_TOKEN)));
pub const APPEND_NO_TOKEN: AppendHeaders<Option<(axum::http::header::HeaderName, &str)>> =
    AppendHeaders(None);

/// Number of base-64 characters in the device code used for authentication.
///
/// These must be unique and cryptographically secure because we don't check for
/// overlaps.
const DEVICE_CODE_LEN: usize = 64;

/// How long a Discord authentication request is valid for.
const DISCORD_OTP_TIMEOUT: Duration = Duration::from_secs(15 * 60); // 15 minutes
/// How long an email authentication request is valid for.
const EMAIL_OTP_TIMEOUT: TimeDelta = TimeDelta::minutes(15);
/// Number of base-10 characters in the user code used for authentication.
const OTP_LEN: usize = 6;

/// Redirect URL to user settings page.
const SETTINGS_PAGE: &str = "/settings";

/// Contact method confirmation, used for logging in or confirming login/contact
/// methods.
///
/// Example:
///
/// 1. The user loads the login page, enters their email address, and clicks
///    "log in."
/// 2. The web browser sends a request to the server to authenticate via email.
/// 3. The server generates a [`Otp`]. `device_code` is sent to the browser and
///    `otp` is sent to the user's email.
/// 4. The user enters `otp` into the browser.
/// 5. The browser sends `device_code` and `otp` to the server.
/// 6. The server responds with a token or a redirect.
pub struct Otp {
    /// Code sent to the browser that issued the contact method confirmation
    /// request.
    pub device_code: String,
    /// One-time passcode sent via the contact method used to confirm
    /// authentication.
    pub otp: String,
    /// Contact method by which to authenticate the user.
    pub contact: AuthContact,
    /// Time when the request expires.
    pub expiry: DateTime<Utc>,
    /// Whether the request has been confirmed.
    pub confirmed: bool,
    /// Callback to run if the request is confirmed.
    pub action: AuthConfirmAction,
}
impl Otp {
    pub fn new(contact: AuthContact, action: AuthConfirmAction) -> Self {
        let expiry = match &contact {
            AuthContact::Email(_) => Utc::now() + EMAIL_OTP_TIMEOUT,
            AuthContact::Discord(_) => Utc::now() + DISCORD_OTP_TIMEOUT,
        };
        Self {
            device_code: crate::util::random_b64_string(DEVICE_CODE_LEN),
            otp: crate::util::random_digits_string(OTP_LEN),
            contact,
            expiry,
            confirmed: false,
            action,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expiry
    }
}

/// Contact method by which to authenticate a user.
#[derive(Debug, Clone)]
pub enum AuthContact {
    /// Email address.
    Email(String),
    /// Discord ID.
    Discord(u64),
}
impl fmt::Display for AuthContact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthContact::Email(email) => write!(f, "email {email}"),
            AuthContact::Discord(discord_id) => write!(f, "discord {discord_id}"),
        }
    }
}

/// Type of contact method by which to authenticate a user.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AuthType {
    /// Email containing one-time code.
    EmailOtp,
    /// Discord DM containing one-time code.
    DiscordOtp,
}

#[derive(Debug)]
pub enum AuthConfirmAction {
    SignIn {
        account_exists: bool,
        redirect: Option<String>,
    },
    ChangeEmail {
        editor: User,
        target: UserId,
        new_email: String,
    },
    ChangeDiscordId {
        editor: User,
        target: UserId,
        new_discord_id: u64,
    },
}
impl AuthConfirmAction {
    fn action_str(&self) -> &'static str {
        match self {
            AuthConfirmAction::SignIn { account_exists, .. } if *account_exists => "sign in",
            AuthConfirmAction::SignIn { .. } => "create an account",
            AuthConfirmAction::ChangeEmail { .. } => "change your email address",
            AuthConfirmAction::ChangeDiscordId { .. } => "change your Discord account",
        }
    }
}

pub struct AuthConfirmResponse {
    pub token_string: Option<String>,
    pub redirect: String,
}

impl IntoResponse for AuthConfirmResponse {
    fn into_response(self) -> Response {
        let mut jar = CookieJar::new();
        if let Some(token_string) = self.token_string {
            jar = jar.add(
                Cookie::build(("token", token_string))
                    .http_only(true)
                    .secure(true)
                    .same_site(SameSite::Strict),
            );
        }

        // assume the query parameter is a relative url, which if js/form.js is doing its job will be
        (jar, Redirect::to(&self.redirect)).into_response()
    }
}

impl AppState {
    /// Confirms an authentication request and returns the action that should be
    /// performed.
    pub async fn confirm_otp(
        &self,
        device_code: &str,
        otp: &str,
    ) -> AppResult<AuthConfirmResponse> {
        // Cleans very old requests, but keep slightly-expired ones
        self.clean_auth_requests().await;

        let mut auth_requests = self.otps.lock().await;

        let req = auth_requests
            .get_mut(device_code)
            .filter(|req| req.otp == otp)
            .filter(|req| !req.confirmed) // no replay attacks!
            .ok_or(AppError::InvalidOtp)?;

        if req.is_expired() {
            return Err(AppError::AuthenticationTimeout);
        }

        req.confirmed = true;

        match &req.action {
            AuthConfirmAction::SignIn {
                account_exists: _,
                redirect,
            } => {
                let user = match req.contact.clone() {
                    AuthContact::Email(email) => self.get_or_create_user_with_email(email).await?,
                    AuthContact::Discord(discord_id) => {
                        self.get_or_create_user_with_discord_id(discord_id).await?
                    }
                };
                let token = self.create_token(user.id).await?;
                Ok(AuthConfirmResponse {
                    token_string: Some(token.string),
                    redirect: redirect
                        .clone()
                        .unwrap_or_else(|| user.to_public().relative_url()),
                })
            }
            AuthConfirmAction::ChangeEmail {
                editor,
                target,
                new_email,
            } => {
                self.update_user_email(editor, *target, Some(new_email.clone()))
                    .await?;
                Ok(AuthConfirmResponse {
                    token_string: None,
                    redirect: SETTINGS_PAGE.to_string(),
                })
            }
            AuthConfirmAction::ChangeDiscordId {
                editor,
                target,
                new_discord_id,
            } => {
                self.update_user_discord_id(editor, *target, Some(*new_discord_id))
                    .await?;
                Ok(AuthConfirmResponse {
                    token_string: None,
                    redirect: SETTINGS_PAGE.to_string(),
                })
            }
        }
    }

    /// Removes very old authentication requests. Slightly-expired requests are
    /// retained so we can give a timeout error.
    pub async fn clean_auth_requests(&self) {
        // Keep requests for an extra day to give accurate status
        let now = Utc::now();
        self.otps
            .lock()
            .await
            .retain(|_, req| now < req.expiry + TimeDelta::days(1));
    }

    /// Generates and sends an OTP via `contact` and returns the device code.
    pub async fn initiate_auth(
        &self,
        contact: AuthContact,
        action: AuthConfirmAction,
    ) -> AppResult<String> {
        let action_str = action.action_str();

        // Check authorization.
        match &action {
            AuthConfirmAction::SignIn { .. } => (), // always allow

            AuthConfirmAction::ChangeEmail { editor, target, .. }
            | AuthConfirmAction::ChangeDiscordId { editor, target, .. } => {
                editor.try_edit_auth(*target)?;
            }
        }

        // Create OTP.
        let new_auth_confirm = Otp::new(contact.clone(), action);
        let device_code = new_auth_confirm.device_code.clone();
        let otp = new_auth_confirm.otp.clone();
        self.otps
            .lock()
            .await
            .insert(device_code.clone(), new_auth_confirm);

        // Send OTP.
        tracing::info!("sending OTP to {contact}");
        let msg_template_params = serde_json::json!({
            "otp": otp,
            "action": action_str,
            "domain_name": *crate::env::DOMAIN_NAME,
            "support_email": *crate::env::SUPPORT_EMAIL
        });
        match contact {
            AuthContact::Email(email) => {
                crate::email::send_email(
                    &email,
                    "Hypercubing leaderboards authentication request",
                    &crate::render_template("messages/otp.txt", &msg_template_params)?,
                    &crate::render_template("messages/otp.html", &msg_template_params)?,
                )
                .await?;
            }
            AuthContact::Discord(discord_id) => {
                use poise::serenity_prelude::*;

                let discord = self.try_discord()?;

                let user = UserId::new(discord_id);
                let user_dms = user.create_dm_channel(discord).await?;

                let msg_content = crate::render_template("messages/otp.md", &msg_template_params)?;
                let msg = CreateMessage::new().content(&msg_content);

                user_dms.send_message(discord, msg).await?;
            }
        }

        Ok(device_code)
    }

    pub async fn invalidate_current_token(&self, jar: &CookieJar) -> AppResult<impl IntoResponse> {
        // it can't be RequestBody because it needs the token

        let Some(token) = jar.get("token") else {
            return Ok((APPEND_NO_TOKEN, "not signed in"));
        };
        let token = token.value();
        self.remove_token(token).await?;
        Ok((APPEND_EXPIRED_TOKEN, "ok"))
    }
}
