use axum::body::Body;
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::CookieJar;

use crate::db::user::User;
use crate::error::AppError;
use crate::traits::RequestBody;
use crate::AppState;

#[derive(serde::Deserialize)]
pub struct SignOutPage {
    redirect: Option<String>,
}

pub struct SignOutPageResponse {
    user: Option<User>,
    redirect: Option<String>,
}

impl RequestBody for SignOutPage {
    type Response = SignOutPageResponse;

    async fn preprocess_jar(state: &AppState, jar: &CookieJar) -> Result<(), AppError> {
        crate::api::auth::invalidate_current_token(state, jar).await?;
        Ok(())
    }

    async fn request(
        self,
        _state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        Ok(SignOutPageResponse {
            user,
            redirect: self.redirect,
        })
    }
}

impl IntoResponse for SignOutPageResponse {
    fn into_response(self) -> Response<Body> {
        Redirect::to(self.redirect.as_deref().unwrap_or("/")).into_response()
    }
}
