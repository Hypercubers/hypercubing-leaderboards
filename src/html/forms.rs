use axum::response::Response;

use crate::db::User;
use crate::error::AppError;
use crate::{AppState, RequestBody};

#[derive(serde::Deserialize)]
pub struct UploadSolveExternal {}

impl RequestBody for UploadSolveExternal {
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

        // let mut program_versions = state.get_all_representations().await?;
        // program_versions.sort_by_key(|p| (p.name()));

        Ok(crate::render_html_template(
            "index.html",
            &user,
            serde_json::json!({
                "puzzles": puzzles,
                // "program_versions": program_versions,
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
