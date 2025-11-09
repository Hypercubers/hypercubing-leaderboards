use axum::response::Response;

use crate::db::{EditAuthorization, SolveId, User};
use crate::{AppError, AppState, RequestBody};

#[derive(serde::Deserialize)]
pub struct EditSolvePage {
    id: SolveId,
}

impl RequestBody for EditSolvePage {
    type Response = Response;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let user = user.ok_or(AppError::NotLoggedIn)?;
        let solve = state.get_solve(self.id).await?;
        user.try_edit_auth(&solve)?;

        let puzzles = state.get_all_puzzles().await?;
        let variants = state.get_all_variants().await?;
        let programs = state.get_all_programs().await?;

        Ok(crate::render_html_template(
            "edit-solve.html",
            &Some(user),
            serde_json::json!({
                "puzzles": puzzles,
                "variants": variants,
                "programs": programs,
                "solve": solve,
            }),
        ))
    }
}
