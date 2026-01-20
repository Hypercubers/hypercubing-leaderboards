use axum::response::IntoResponse;

use super::solve_table::{
    LeaderboardTableColumns, LeaderboardTableRows, SolveTableRow, SolvesTable, SolvesTablesResponse,
};
use crate::db::{PublicUser, User, UserId};
use crate::traits::{Linkable, RequestBody};
use crate::{AppError, AppState};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct MySubmissionsPage {}

pub struct MySubmissionsPageResponse {
    user: Option<User>,
}

impl RequestBody for MySubmissionsPage {
    type Response = MySubmissionsPageResponse;

    async fn request(
        self,
        _state: AppState,
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

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SolverSubmissionsPage {
    pub id: UserId,
}

pub struct SolverSubmissionsPageResponse {
    user: Option<User>,
    target_user: PublicUser,
}

impl RequestBody for SolverSubmissionsPage {
    type Response = SolverSubmissionsPageResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        if !user.as_ref().ok_or(AppError::NotLoggedIn)?.moderator {
            return Err(AppError::NotAuthorized);
        }
        let target_user = state
            .get_opt_user(self.id)
            .await?
            .ok_or(AppError::NotFound)?
            .to_public();
        Ok(SolverSubmissionsPageResponse { user, target_user })
    }
}

impl IntoResponse for SolverSubmissionsPageResponse {
    fn into_response(self) -> axum::response::Response {
        let target_user_name = self.target_user.display_name();
        crate::render_html_template(
            "solver-submissions.html",
            &self.user,
            serde_json::json!({
                "target_user": self.target_user,
                "target_user_name": target_user_name,
            }),
        )
    }
}

#[derive(serde::Deserialize)]
pub struct SolverSubmissionsTable {
    id: UserId,
}

impl RequestBody for SolverSubmissionsTable {
    type Response = SolvesTablesResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let user = user.ok_or(AppError::NotLoggedIn)?;
        if !(user.id == self.id || user.moderator) {
            return Err(AppError::NotAuthorized);
        }

        let solves = state
            .get_solver_submissions(self.id)
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

        Ok(SolvesTable {
            heading: None,
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
        }
        .into())
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct PendingSubmissionsPage {}

pub struct PendingSubmissionsPageResponse {
    user: Option<User>,
    autoverify_queue: Vec<serde_json::Value>,
}

impl RequestBody for PendingSubmissionsPage {
    type Response = PendingSubmissionsPageResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        if user.is_none() {
            return Err(AppError::NotLoggedIn);
        }

        let mut autoverify_queue = vec![];
        for solve_id in state.autoverifier.queue_snapshot().await {
            let solve = state.get_solve(solve_id).await?;
            autoverify_queue.push(serde_json::json!({
                "id": solve_id,
                "link": solve.relative_url(),
                "solver_name": solve.solver.display_name(),
                "solver_link": solve.solver.relative_url(),
                "upload_date": solve.upload_date,
            }));
        }

        Ok(PendingSubmissionsPageResponse {
            user,
            autoverify_queue,
        })
    }
}

impl IntoResponse for PendingSubmissionsPageResponse {
    fn into_response(self) -> axum::response::Response {
        crate::render_html_template(
            "pending-submissions.html",
            &self.user,
            serde_json::json!({ "autoverify_queue": self.autoverify_queue }),
        )
    }
}

#[derive(serde::Deserialize)]
pub struct PendingSubmissionsTable {}

impl RequestBody for PendingSubmissionsTable {
    type Response = SolvesTablesResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let user = user.ok_or(AppError::NotLoggedIn)?;
        if !user.moderator {
            return Err(AppError::NotAuthorized);
        }

        let solves = state
            .get_pending_submissions()
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

        Ok(SolvesTable {
            heading: None,
            table_rows: LeaderboardTableRows::Solves(solves),
            columns: LeaderboardTableColumns {
                puzzle: true,
                rank: false,
                solver: true,
                record_holder: false,
                speed_cs: true,
                move_count: true,
                verified: true,
                date: true,
                program: true,
                total_solvers: false,
                score: false,
            },
        }
        .into())
    }
}
