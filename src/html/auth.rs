use crate::api::auth::TokenReturn;
use crate::api::auth::UserRequestOtpResponse;
use crate::api::auth::{UserRequestOtp, UserRequestToken};
use crate::db::User;
use crate::error::AppError;
use crate::traits::RequestResponse;
use crate::AppState;
use crate::RequestBody;
use axum::response::IntoResponse;
use axum::response::Response;
use axum_typed_multipart::TryFromMultipart;

#[derive(serde::Deserialize, TryFromMultipart)]
pub struct SignInForm {
    email: String,
    otp: Option<String>,
}

struct SignInResponse {
    response: Response,
}

impl RequestBody for SignInForm {
    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<impl RequestResponse, AppError> {
        Ok(SignInResponse {
            response: match self.otp {
                None => UserRequestOtp {
                    email: self.email,
                    display_name: None,
                }
                .request(state, user)
                .await?
                .as_axum_response()
                .await
                .into_response(),
                Some(otp_code) => UserRequestToken {
                    email: self.email,
                    otp_code,
                }
                .request(state, user)
                .await?
                .as_axum_response()
                .await
                .into_response(),
            },
        })
    }
}

impl RequestResponse for SignInResponse {
    async fn as_axum_response(self) -> impl IntoResponse {
        self.response
    }
}
