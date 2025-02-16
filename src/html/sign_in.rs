use axum::body::Body;
use axum::response::{Html, IntoResponse, Redirect, Response};

use crate::db::user::User;
use crate::error::AppError;
use crate::traits::{Linkable, RequestBody};
use crate::{AppState, HBS};

#[derive(serde::Deserialize)]
pub struct SignInPage {}

pub struct SignInPageResponse {
    user: Option<User>,
}

impl RequestBody for SignInPage {
    type Response = SignInPageResponse;

    async fn request(
        self,
        _state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        Ok(SignInPageResponse { user })
    }
}

impl IntoResponse for SignInPageResponse {
    fn into_response(self) -> Response<Body> {
        if let Some(user) = self.user {
            Redirect::to(&user.to_public().relative_url()).into_response()
        } else {
            Html(
                HBS
                    .render(
                        "sign-in.html",
                        &serde_json::json!({
                            "active_user": self.user.map(|u|u.to_public().to_header_json()).unwrap_or_default()
                        }),
                    )
                    .expect("render error"),
            )
            .into_response()
        }
    }
}
