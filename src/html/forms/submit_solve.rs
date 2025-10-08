use axum::response::Response;

use crate::db::User;
use crate::{AppError, AppState, RequestBody};

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

        let puzzles = state.get_all_puzzles().await?;
        let variants = state.get_all_variants().await?;
        let programs = state.get_all_programs().await?;

        Ok(crate::render_html_template(
            "submit-solve.html",
            &user,
            serde_json::json!({
                "puzzles": puzzles,
                "variants": variants,
                "programs": programs,
            }),
        ))
    }
}
