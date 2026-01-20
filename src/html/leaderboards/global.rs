use std::collections::HashMap;

use axum::body::Body;
use axum::http::Response;
use axum::response::IntoResponse;
use itertools::Itertools;

use super::LeaderboardEvent;
use crate::db::{CategoryQuery, MainPageCategory, ProgramQuery, ScoreQuery, User, VariantQuery};
use crate::html::solve_table::{
    LeaderboardTableColumns, LeaderboardTableRows, SolveTableRow, SolvesTable,
    SolvesTablesResponse, UserTableRow,
};
use crate::traits::{Linkable, RequestBody};
use crate::{AppError, AppState};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct GlobalLeaderboardTable {
    pub event: Option<LeaderboardEvent>,
    pub filters: Option<bool>,
    pub macros: Option<bool>,
    pub variant: Option<VariantQuery>,
    pub program: Option<ProgramQuery>,
}
impl GlobalLeaderboardTable {
    pub fn global_leaderboard_query(&self) -> GlobalLeaderboardQuery {
        let event = self.event.unwrap_or_default();

        match event {
            LeaderboardEvent::Single
            | LeaderboardEvent::Avg
            | LeaderboardEvent::Bld
            | LeaderboardEvent::Oh => CategoryQuery::Speed {
                average: event == LeaderboardEvent::Avg,
                blind: event == LeaderboardEvent::Bld,
                filters: self.filters,
                macros: self.macros,
                one_handed: event == LeaderboardEvent::Oh,
                variant: self.variant.clone().unwrap_or(VariantQuery::All),
                program: self.program.clone().unwrap_or(ProgramQuery::All),
            }
            .into(),
            LeaderboardEvent::Fmc | LeaderboardEvent::FmcCa => CategoryQuery::Fmc {
                computer_assisted: event == LeaderboardEvent::FmcCa,
            }
            .into(),
            LeaderboardEvent::Distinct => ScoreQuery::Distinct.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GlobalLeaderboardQuery {
    Category(CategoryQuery),
    Score(ScoreQuery),
}
impl From<CategoryQuery> for GlobalLeaderboardQuery {
    fn from(value: CategoryQuery) -> Self {
        Self::Category(value)
    }
}
impl From<ScoreQuery> for GlobalLeaderboardQuery {
    fn from(value: ScoreQuery) -> Self {
        Self::Score(value)
    }
}

impl RequestBody for GlobalLeaderboardTable {
    type Response = SolvesTablesResponse;

    async fn request(
        self,
        state: crate::AppState,
        _user: Option<crate::db::User>,
    ) -> Result<Self::Response, crate::AppError> {
        match self.global_leaderboard_query() {
            GlobalLeaderboardQuery::Category(query) => {
                let solver_counts: HashMap<MainPageCategory, i64> = state
                    .get_all_puzzles_counts(&query)
                    .await?
                    .into_iter()
                    .collect();

                let solves = state.get_all_puzzles_leaderboard(&query).await?;

                let rows = solves
                    .into_iter()
                    .map(|(solve_event, solve)| {
                        let total_solvers = *solver_counts
                            .get(&match query {
                                CategoryQuery::Speed { .. } => MainPageCategory::Speed {
                                    puzzle: solve.puzzle.id,
                                    variant: solve.variant.as_ref().map(|v| v.id),
                                    material: solve.program.material,
                                },
                                CategoryQuery::Fmc { .. } => MainPageCategory::Fmc {
                                    puzzle: solve.puzzle.id,
                                },
                            })
                            .unwrap_or(&0);

                        SolveTableRow::new(&solve_event, &solve, None, Some(total_solvers), &query)
                    })
                    .sorted_by_key(|row| row.total_solvers.map(|n| -n))
                    .collect();

                Ok(SolvesTable {
                    heading: None,
                    table_rows: LeaderboardTableRows::Solves(rows),
                    columns: LeaderboardTableColumns {
                        puzzle: true,
                        rank: false,
                        solver: false,
                        record_holder: true,
                        speed_cs: matches!(query, CategoryQuery::Speed { .. }),
                        move_count: matches!(query, CategoryQuery::Fmc { .. }),
                        verified: false,
                        date: true,
                        program: true,
                        total_solvers: true,
                        score: false,
                    },
                }
                .grouped())
            }
            GlobalLeaderboardQuery::Score(query) => {
                let users_and_scores = state.get_score_leaderboard(query).await?;

                let rows = users_and_scores
                    .into_iter()
                    .map(|(rank, user, count)| UserTableRow {
                        rank,
                        solver_name: user.display_name(),
                        solver_url: user.relative_url(),
                        score: count,
                    })
                    .collect();

                Ok(SolvesTable {
                    heading: None,
                    table_rows: LeaderboardTableRows::Users(rows),
                    columns: LeaderboardTableColumns {
                        puzzle: false,
                        rank: true,
                        solver: true,
                        record_holder: false,
                        speed_cs: false,
                        move_count: false,
                        verified: false,
                        date: false,
                        program: false,
                        total_solvers: false,
                        score: true,
                    },
                }
                .grouped())
            }
        }
    }
}

impl IntoResponse for SolvesTablesResponse {
    fn into_response(self) -> axum::response::Response {
        crate::render_html_template(
            "components/solve-table-contents.html",
            &None,
            serde_json::to_value(&self).unwrap_or_default(),
        )
    }
}

#[derive(serde::Deserialize)]
pub struct GlobalLeaderboard {}

pub struct GlobalLeaderboardResponse {
    user: Option<User>,
    pending_submissions_count: Option<i64>,
}

impl RequestBody for GlobalLeaderboard {
    type Response = GlobalLeaderboardResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let pending_submissions_count = if user.as_ref().is_some_and(|u| u.moderator) {
            state.get_pending_submissions_count().await?
        } else {
            None
        };

        Ok(GlobalLeaderboardResponse {
            user,
            pending_submissions_count,
        })
    }
}

impl IntoResponse for GlobalLeaderboardResponse {
    fn into_response(self) -> Response<Body> {
        crate::render_html_template(
            "index.html",
            &self.user,
            serde_json::json!({
                "pending_submissions_count": self.pending_submissions_count,
            }),
        )
    }
}
