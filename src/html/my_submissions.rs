use std::collections::HashMap;

use axum::response::IntoResponse;
use futures::TryFutureExt;
use itertools::Itertools;

use crate::db::{
    Category, CategoryQuery, Event, MainPageCategory, ProgramQuery, RankedFullSolve, User, UserId,
    VariantQuery,
};
use crate::error::AppError;
use crate::traits::RequestBody;
use crate::AppState;

use super::global_leaderboard::{
    GlobalLeaderboardQuery, GlobalLeaderboardTable, LeaderboardEvent, LeaderboardTableColumns,
    LeaderboardTableRows, SolveTableRow, SolvesTableResponse,
};

#[derive(serde::Deserialize)]
pub struct MySubmissionsPage {}

pub struct MySubmissionsPageResponse {
    user: Option<User>,
}

impl RequestBody for MySubmissionsPage {
    type Response = MySubmissionsPageResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        if user.is_none() {
            return Err(AppError::NotLoggedIn);
        }

        Ok(MySubmissionsPageResponse { user })
    }
}

impl IntoResponse for MySubmissionsPageResponse {
    fn into_response(self) -> axum::response::Response {
        crate::render_html_template("my-submissions.html", &self.user, serde_json::json!({}))
    }
}

#[derive(serde::Deserialize)]
pub struct MySubmissionsTable {}

impl RequestBody for MySubmissionsTable {
    type Response = SolvesTableResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let user = user.ok_or(AppError::NotLoggedIn)?;

        let solves = state
            .get_solver_submissions(user.id)
            .await?
            .into_iter()
            .map(|solve| {
                SolveTableRow::new(
                    &solve.primary_event(),
                    &solve,
                    None,
                    None,
                    &solve.primary_category_query(),
                )
            })
            .collect();

        Ok(SolvesTableResponse {
            table_rows: LeaderboardTableRows::Solves(solves),
            columns: LeaderboardTableColumns {
                puzzle: true,
                rank: false,
                solver: false,
                record_holder: false,
                speed_cs: true,
                move_count: true,
                verified: true,
                date: true,
                program: true,
                total_solvers: false,
                score: false,
            },
        })
    }
}
