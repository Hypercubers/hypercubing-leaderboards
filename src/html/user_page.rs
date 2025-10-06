use std::collections::HashMap;

use axum::body::Body;
use axum::response::{IntoResponse, Response};
use itertools::Itertools;

use super::leaderboards::global::{GlobalLeaderboardQuery, GlobalLeaderboardTable};
use super::solve_table::{
    LeaderboardTableColumns, LeaderboardTableRows, SolveTableRow, SolvesTableResponse,
};
use crate::db::{
    Category, CategoryQuery, Event, MainPageCategory, ProgramQuery, RankedFullSolve, User, UserId,
    VariantQuery,
};
use crate::html::leaderboards::LeaderboardEvent;
use crate::traits::RequestBody;
use crate::{AppError, AppState};

#[derive(serde::Deserialize)]
pub struct SolverLeaderboard {
    id: UserId,
}

pub struct SolverLeaderboardResponse {
    target_user: User,
    can_edit: bool,
    user: Option<User>,
    pending_submissions_count: Option<i64>,
}

impl RequestBody for SolverLeaderboard {
    type Response = SolverLeaderboardResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let target_user = state
            .get_opt_user(self.id)
            .await?
            .ok_or(AppError::InvalidQuery(format!(
                "Solver with id {} does not exist",
                self.id.0
            )))?;

        let can_edit = user
            .as_ref()
            .is_some_and(|editor| editor.edit_auth(target_user.id).is_some());

        let pending_submissions_count = if user.as_ref().is_some_and(|u| u.moderator) {
            state
                .get_pending_submissions_count_for_user(target_user.id)
                .await?
        } else {
            None
        };

        Ok(SolverLeaderboardResponse {
            target_user,
            can_edit,
            user,
            pending_submissions_count,
        })
    }
}

impl IntoResponse for SolverLeaderboardResponse {
    fn into_response(self) -> Response<Body> {
        let target_user_name = self.target_user.to_public().display_name();
        crate::render_html_template(
            "solver.html",
            &self.user,
            serde_json::json!({
                "target_user": self.target_user,
                "target_user_name": target_user_name,
                "can_edit": self.can_edit,
                "pending_submissions_count": self.pending_submissions_count,
            }),
        )
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SolverLeaderboardTable {
    pub id: UserId,

    pub event: Option<LeaderboardEvent>,
    pub filters: Option<bool>,
    pub macros: Option<bool>,

    pub variant: Option<VariantQuery>,
    pub program: Option<ProgramQuery>,

    #[serde(default)]
    pub history: bool,
}

impl RequestBody for SolverLeaderboardTable {
    type Response = SolvesTableResponse;

    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let global = GlobalLeaderboardTable {
            event: self.event,
            filters: self.filters,
            macros: self.macros,
            variant: self.variant,
            program: self.program,
        };
        let GlobalLeaderboardQuery::Category(category_query) = global.global_leaderboard_query()
        else {
            return Err(AppError::InvalidQuery("bad category".to_string()));
        };
        let total_solvers: HashMap<MainPageCategory, i64> = state
            .get_all_puzzles_counts(&category_query)
            .await?
            .into_iter()
            .collect();
        // let puzzle = state.get_puzzle(self.id).await?.ok_or(AppError::NotFound)?;

        let solves = state.get_solver_pbs(self.id, &category_query).await?;

        let solve_rows = solves
            .into_iter()
            .sorted_by_key(|(category, _solve)| *total_solvers.get(category).unwrap_or(&0))
            .map(|(_category, RankedFullSolve { rank, solve })| {
                let event = Event {
                    puzzle: solve.puzzle.clone(),
                    category: match &category_query {
                        CategoryQuery::Speed {
                            average,
                            blind,
                            filters,
                            macros,
                            one_handed,
                            variant: _,
                            program: _,
                        } => {
                            let default_filters = match &solve.variant {
                                Some(v) => v.primary_filters,
                                None => solve.puzzle.primary_filters,
                            };
                            let default_macros = match &solve.variant {
                                Some(v) => v.primary_macros,
                                None => solve.puzzle.primary_macros,
                            };
                            Category::Speed {
                                average: *average,
                                blind: *blind,
                                filters: filters.unwrap_or(default_filters),
                                macros: macros.unwrap_or(default_macros),
                                one_handed: *one_handed,
                                variant: solve.variant.clone(),
                                material: solve.program.material,
                            }
                        }

                        CategoryQuery::Fmc { computer_assisted } => Category::Fmc {
                            computer_assisted: *computer_assisted,
                        },
                    },
                };
                SolveTableRow::new(&event, &solve, Some(rank), None, &category_query)
            })
            .collect();

        Ok(SolvesTableResponse {
            table_rows: LeaderboardTableRows::Solves(solve_rows),

            columns: LeaderboardTableColumns {
                puzzle: true,
                rank: !self.history,
                solver: false,
                record_holder: false,
                speed_cs: matches!(category_query, CategoryQuery::Speed { .. }),
                move_count: matches!(category_query, CategoryQuery::Fmc { .. }),
                verified: false,
                date: true,
                program: true,
                total_solvers: false,
                score: false,
            },
        })
    }
}
