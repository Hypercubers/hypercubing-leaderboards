use crate::db::program::ProgramVersion;
use crate::db::puzzle::Puzzle;
pub use crate::db::solve::FullSolve;
use crate::db::solve::SolveId;
use crate::db::user::User;
use crate::error::AppError;
use crate::traits::RequestBody;
use crate::AppState;
use crate::HBS;
use axum::body::Body;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;

#[derive(serde::Deserialize)]
pub struct SolvePage {
    id: SolveId,
}

pub struct SolvePageResponse {
    can_edit: bool,
    puzzles: Vec<Puzzle>,
    program_versions: Vec<ProgramVersion>,
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
        let mut solve = state
            .get_full_solve(self.id)
            .await?
            .ok_or(AppError::InvalidQuery("no such solve".to_string()))?;

        let edit_auth = solve.can_edit_opt(user.as_ref());
        if edit_auth.is_none() {
            solve = state
                .get_leaderboard_solve(self.id)
                .await?
                .ok_or(AppError::InvalidQuery("no such solve".to_string()))?;
        }
        let solve = solve;

        let mut puzzles = state.get_all_puzzles().await?;
        puzzles.sort_by_key(|p| p.name.clone());

        let mut program_versions = state.get_all_program_versions().await?;
        program_versions.sort_by_key(|p| (p.name()));

        Ok(SolvePageResponse {
            can_edit: edit_auth.is_some(),
            puzzles,
            program_versions,
            solve,
            user,
        })
    }
}

impl IntoResponse for SolvePageResponse {
    fn into_response(self) -> Response<Body> {
        Html(
            HBS
                .render(
                    "solve.html",
                    &serde_json::json!({
                        "solve": self.solve,
                        "can_edit": self.can_edit,
                        "user_url": self.solve.user().url_path(),
                        "user_name": self.solve.user().name(),
                        "puzzle_url": self.solve.puzzle_category().url_path(),
                        "puzzle_name": self.solve.puzzle_category().base.name(),
                        "puzzles": self.puzzles,
                        "program_versions": self.program_versions,
                        "program": self.solve.program_version().name(),
                        "active_user": self.user.map(|u|u.to_public().to_header_json()).unwrap_or_default(),
                    }),
                )
                .expect("render error"),
        )
        .into_response()
    }
}
