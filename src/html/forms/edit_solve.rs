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
        let auth = user.try_edit_auth(&solve)?;

        let mut puzzles = state.get_all_puzzles().await?;
        puzzles.sort_by_key(|p| p.name.clone());

        let variants = state.get_all_variants().await?;

        let mut programs = state.get_all_programs().await?;
        programs.sort_by_key(|p| (!p.material, p.name.clone()));

        Ok(crate::render_html_template(
            "edit-solve.html",
            &Some(user),
            serde_json::json!({
                "puzzles": puzzles,
                "variants": variants,
                "programs": programs,
                "moderator": auth == EditAuthorization::Moderator,
                "solve": solve,
            }),
        ))
    }
}
