use axum::response::{IntoResponse, Response};
use axum_typed_multipart::TryFromMultipart;

use crate::api::auth::{UserRequestOtp, UserRequestToken};
use crate::db::User;
use crate::error::AppError;
use crate::{AppState, RequestBody};

#[derive(serde::Deserialize, TryFromMultipart)]
pub struct RequestOtp {
    email: String,
    #[form_data(field_name = "cf-turnstile-response")]
    turnstile_response: Option<String>,
    redirect: Option<String>,
}
impl RequestBody for RequestOtp {
    type Response = Response;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        tracing::info!("requesting OTP");

        Ok(UserRequestOtp {
            email: self.email,
            turnstile_response: self.turnstile_response,
            redirect: self.redirect,
        }
        .request(state, user)
        .await?
        .into_response())
    }
}

#[derive(serde::Deserialize, TryFromMultipart)]
pub struct SignInOtp {
    email: String,
    otp: String,
    redirect: Option<String>,
}
impl RequestBody for SignInOtp {
    type Response = Response;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        Ok(UserRequestToken {
            email: self.email,
            otp_code: self.otp,
            redirect: self.redirect,
        }
        .request(state, user)
        .await?
        .into_response())
    }
}
