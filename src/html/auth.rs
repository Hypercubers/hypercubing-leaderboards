use axum::body::Body;
use axum::response::{IntoResponse, Response};
use axum_typed_multipart::TryFromMultipart;

use crate::api::auth::{UserRequestOtp, UserRequestToken};
use crate::db::User;
use crate::error::AppError;
use crate::{AppState, RequestBody};

#[derive(serde::Deserialize, TryFromMultipart)]
pub struct SignInForm {
    email: String,
    otp: Option<String>,
}

pub struct SignInResponse {
    response: Response,
}

impl RequestBody for SignInForm {
    type Response = SignInResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        Ok(SignInResponse {
            response: match self.otp {
                None => UserRequestOtp {
                    email: self.email,
                    display_name: None,
                }
                .request(state, user)
                .await?
                .into_response(),
                Some(otp_code) => UserRequestToken {
                    email: self.email,
                    otp_code,
                }
                .request(state, user)
                .await?
                .into_response(),
            },
        })
    }
}

impl IntoResponse for SignInResponse {
    fn into_response(self) -> Response<Body> {
        self.response
    }
}
