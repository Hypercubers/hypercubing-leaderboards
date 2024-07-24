use crate::db::program::ProgramVersion;
use crate::db::puzzle::Puzzle;
pub use crate::db::solve::LeaderboardSolve;
use crate::db::user::User;
use crate::error::AppError;
use crate::traits::RequestBody;
use crate::AppState;
use axum::body::Body;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;

#[derive(serde::Deserialize)]
pub struct SolvePage {
    id: i32,
}

pub struct SolvePageResponse {
    can_edit: bool,
    puzzles: Vec<Puzzle>,
    program_versions: Vec<ProgramVersion>,
    solve: LeaderboardSolve,
}

impl RequestBody for SolvePage {
    type Response = SolvePageResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let solve = state
            .get_leaderboard_solve(self.id)
            .await?
            .ok_or(AppError::InvalidQuery("no such solve".to_string()))?;

        if !(solve.valid_solve || solve.can_edit_opt(user.as_ref()).is_some()) {
            return Err(AppError::InvalidQuery("no such solve".to_string()));
        }

        let mut puzzles = state.get_all_puzzles().await?;
        puzzles.sort_by_key(|p| p.name.clone());

        let mut program_versions = state.get_all_program_versions().await?;
        program_versions.sort_by_key(|p| (p.name()));

        Ok(SolvePageResponse {
            can_edit: solve.can_edit_opt(user.as_ref()).is_some(),
            puzzles,
            program_versions,
            solve,
        })
    }
}

impl IntoResponse for SolvePageResponse {
    fn into_response(self) -> Response<Body> {
        Html(
            crate::hbs!()
                .render(
                    "solve",
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
                    }),
                )
                .expect("render error"),
        )
        .into_response()
    }
}
