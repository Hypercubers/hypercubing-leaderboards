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

impl RequestBody for SignOutPage {
    type Response = Response;

    async fn preprocess_jar(state: &AppState, jar: &CookieJar) -> Result<(), AppError> {
        crate::api::auth::invalidate_current_token(state, jar).await?;
        Ok(())
    }

    async fn request(
        self,
        _state: AppState,
        _user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        Ok(Redirect::to(self.redirect.as_deref().unwrap_or("/")).into_response())
    }
}
