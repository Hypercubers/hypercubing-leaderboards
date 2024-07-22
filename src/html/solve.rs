pub use crate::db::solve::LeaderboardSolve;
use crate::db::user::User;
use crate::error::AppError;
use crate::html::forms::program_version_options;
use crate::html::forms::puzzle_options;
use crate::traits::RequestBody;
use crate::util::render_time;
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
    puzzle_options: String,
    program_version_options: String,
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

        if !solve.valid_solve {
            return Err(AppError::InvalidQuery("no such solve".to_string()));
        }

        Ok(SolvePageResponse {
            can_edit: user.map(|u| u.moderator).unwrap_or(false),
            puzzle_options: puzzle_options(&state).await?,
            program_version_options: program_version_options(&state).await?,
            solve,
        })
    }
}

impl IntoResponse for SolvePageResponse {
    fn into_response(self) -> Response<Body> {
        Html(format!(
            include_str!("../../html/solve.html"),
            cannot_edit = if self.can_edit { "" } else { "cannot-edit" },
            solve_id = self.solve.id,
            user_url = self.solve.user().url_path(),
            user_name = self.solve.user().html_name(),
            puzzle_url = self.solve.puzzle_category().url_path(),
            puzzle_name = self.solve.puzzle_category().base.name(),
            puzzle_options = self.puzzle_options,
            program_version_options = self.program_version_options,
            filters = if self.solve.uses_filters {
                "uses"
            } else {
                "no"
            },
            macros = if self.solve.uses_macros { "uses" } else { "no" },
            filters_checked = if self.solve.uses_filters {
                "checked"
            } else {
                ""
            },
            macros_checked = if self.solve.uses_macros {
                "checked"
            } else {
                ""
            },
            time = self
                .solve
                .speed_cs
                .map(render_time)
                .unwrap_or("-".to_string()),
            speed_cs = self
                .solve
                .speed_cs
                .map(|n| n.to_string())
                .unwrap_or("-".to_string()),
            video_url = self.solve.video_url.clone().unwrap_or("-".to_string()),
            move_count = self
                .solve
                .move_count
                .map(|n| n.to_string())
                .unwrap_or("-".to_string()),
            program = self.solve.program_version().name(),
        ))
        .into_response()
    }
}
