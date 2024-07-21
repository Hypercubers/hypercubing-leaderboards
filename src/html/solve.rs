pub use crate::db::solve::LeaderboardSolve;
use crate::db::user::User;
use crate::error::AppError;
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
    solve: LeaderboardSolve,
}

impl RequestBody for SolvePage {
    type Response = SolvePageResponse;

    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let solve = state
            .get_leaderboard_solve(self.id)
            .await?
            .ok_or(AppError::InvalidQuery("no such solve".to_string()))?;

        if !solve.valid_solve {
            return Err(AppError::InvalidQuery("no such solve".to_string()));
        }

        Ok(SolvePageResponse { solve })
    }
}

impl IntoResponse for SolvePageResponse {
    fn into_response(self) -> Response<Body> {
        Html(format!(
            include_str!("../../html/solve.html"),
            user_name = self.solve.user_html_name(),
            puzzle_name = self.solve.puzzle_category().base.name(),
            filters = if self.solve.uses_filters {
                "uses"
            } else {
                "no"
            },
            macros = if self.solve.uses_macros { "uses" } else { "no" },
            time = self
                .solve
                .speed_cs
                .map(render_time)
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
