use axum::body::Body;
use axum::response::{IntoResponse, Redirect, Response};

use crate::db::user::User;
use crate::error::AppError;
use crate::traits::{Linkable, RequestBody};
use crate::AppState;

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
            crate::render_html_template("sign-in.html", &self.user, serde_json::json!({}))
        }
    }
}
