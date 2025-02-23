use axum::body::Body;
use axum::response::{IntoResponse, Response};

pub use crate::db::FullSolve;
use crate::db::{Puzzle, SolveId, User};
use crate::error::AppError;
use crate::traits::{Linkable, RequestBody};
use crate::AppState;

#[derive(serde::Deserialize)]
pub struct SolvePage {
    id: SolveId,
}

pub struct SolvePageResponse {
    can_edit: bool,
    puzzles: Vec<Puzzle>,
    solve: FullSolve,
    user: Option<User>,
}

impl RequestBody for SolvePage {
    type Response = SolvePageResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let solve = state
            .get_solve(self.id)
            .await?
            .ok_or(AppError::InvalidQuery("no such solve".to_string()))?;

        let edit_auth = solve.can_edit_opt(user.as_ref());

        let mut puzzles = state.get_all_puzzles().await?;
        puzzles.sort_by_key(|p| p.name.clone());

        Ok(SolvePageResponse {
            can_edit: edit_auth.is_some(),
            puzzles,
            solve,
            user,
        })
    }
}

impl IntoResponse for SolvePageResponse {
    fn into_response(self) -> Response<Body> {
        crate::render_html_template(
            "solve.html",
            &self.user,
            serde_json::json!({
                "solve": self.solve,
                "can_edit": self.can_edit,
                "user_url": self.solve.solver.relative_url(),
                "user_name": self.solve.solver.display_name(),
                // "puzzle_url": self.solve.category.speed_relative_url(),
                // "puzzle_name": self.solve.category.base.name(),
                "puzzles": self.puzzles,
                // "program_versions": self.program_versions,
                // "program": self.solve.program_version.name(),
            }),
        )
    }
}
