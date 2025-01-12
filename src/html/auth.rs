use crate::api::auth::{invalidate_current_token, UserRequestOtp, UserRequestToken};
use crate::db::user::User;
use crate::error::AppError;
use crate::AppState;
use crate::RequestBody;
use axum::body::Body;
use axum::extract::State;
use axum::response::Response;
use axum::response::{Html, IntoResponse};
use axum_extra::extract::CookieJar;
use axum_typed_multipart::TryFromMultipart;

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

pub async fn sign_out(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, AppError> {
    invalidate_current_token(State(state), jar).await?;
    Ok(Html(include_str!("../../html/signed-out.html")))
}
