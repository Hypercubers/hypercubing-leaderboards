use crate::api::auth::{user_request_otp, user_request_token, UserRequestOtp, UserRequestToken};
use crate::db::User;
use crate::error::AppError;
use crate::AppState;
use crate::RequestBody;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use axum_typed_multipart::TryFromMultipart;

#[derive(serde::Deserialize, TryFromMultipart)]
pub struct SignInForm {
    email: String,
    otp: Option<String>,
}

impl RequestBody for SignInForm {
    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<impl IntoResponse, AppError> {
        match self.otp {
            None => user_request_otp(
                State(state),
                Json(UserRequestOtp {
                    email: self.email,
                    display_name: None,
                }),
            )
            .await
            .map(|o| o.into_response()),
            Some(otp_code) => user_request_token(
                State(state),
                Json(UserRequestToken {
                    email: self.email,
                    otp_code,
                }),
            )
            .await
            .map(|o| o.into_response()),
        }
    }
}
