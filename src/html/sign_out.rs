use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::CookieJar;

use crate::db::User;
use crate::traits::RequestBody;
use crate::{AppError, AppResult, AppState};

#[derive(serde::Deserialize)]
pub struct SignOutPage {
    redirect: Option<String>,
}

impl RequestBody for SignOutPage {
    type Response = Response;

    async fn preprocess_jar(state: &AppState, jar: &CookieJar) -> AppResult {
        state.invalidate_current_token(jar).await?;
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
