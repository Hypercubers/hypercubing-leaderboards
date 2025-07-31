use axum::response::Response;

use crate::db::User;
use crate::error::AppError;
use crate::{AppState, RequestBody};

#[derive(serde::Deserialize)]
pub struct SubmitSolve {}

impl RequestBody for SubmitSolve {
    type Response = Response;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        if user.is_none() {
            return Err(AppError::NotLoggedIn);
        }

        let mut puzzles = state.get_all_puzzles().await?;
        puzzles.sort_by_key(|p| p.name.clone());

        let variants = state.get_all_variants().await?;

        let mut programs = state.get_all_programs().await?;
        programs.sort_by_key(|p| (!p.material, p.name.clone()));

        Ok(crate::render_html_template(
            "submit.html",
            &user,
            serde_json::json!({
                "puzzles": puzzles,
                "variants": variants,
                "programs": programs,
            }),
        ))
    }
}

#[derive(serde::Deserialize)]
pub struct Settings {}

impl RequestBody for Settings {
    type Response = Response;

    async fn request(
        self,
        _state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        if user.is_none() {
            return Err(AppError::NotLoggedIn);
        }

        Ok(crate::render_html_template(
            "index.html",
            &user,
            serde_json::json!({}),
        ))
    }
}
