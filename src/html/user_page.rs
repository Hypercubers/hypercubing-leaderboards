use axum::body::Body;
use axum::response::{IntoResponse, Response};

use crate::db::{
    Category, CategoryQuery, Event, MainPageQuery, ProgramQuery, Puzzle, PuzzleId, RankedFullSolve,
    User, UserId, VariantId, VariantQuery,
};
use crate::error::AppError;
use crate::traits::RequestBody;
use crate::AppState;

use super::global_leaderboard::{
    GlobalLeaderboardTable, LeaderboardEvent, LeaderboardTableColumns, LeaderboardTableResponse,
    SolveTableRow,
};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SolverLeaderboardTable {
    pub event: Option<LeaderboardEvent>,
    pub filters: Option<bool>,
    pub macros: Option<bool>,

    pub variant: Option<VariantQuery>,
    pub program: Option<ProgramQuery>,

    #[serde(default)]
    pub history: bool,
}

impl RequestBody for SolverLeaderboardTable {
    type Response = LeaderboardTableResponse;

    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let global = GlobalLeaderboardTable {
            event: self.event,
            filters: self.filters,
            macros: self.macros,
        };
        let main_page_query = global.main_page_query();
        // let puzzle = state.get_puzzle(self.id).await?.ok_or(AppError::NotFound)?;

        let category_query = match main_page_query {
            MainPageQuery::Speed {
                average,
                blind,
                filters,
                macros,
                one_handed,
            } => CategoryQuery::Speed {
                average,
                blind,
                filters,
                macros,
                one_handed,
                variant: self.variant.unwrap_or_default(),
                program: self.program.unwrap_or_default(),
            },
            MainPageQuery::Fmc { computer_assisted } => CategoryQuery::Fmc { computer_assisted },
        };

        let solves = vec![];

        Ok(LeaderboardTableResponse {
            table_rows: solves
                .into_iter()
                .map(|RankedFullSolve { rank, solve }| {
                    let event = Event {
                        puzzle: solve.puzzle.clone(),
                        category: match &main_page_query {
                            MainPageQuery::Speed {
                                average,
                                blind,
                                filters,
                                macros,
                                one_handed,
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
                            MainPageQuery::Fmc { computer_assisted } => Category::Fmc {
                                computer_assisted: *computer_assisted,
                            },
                        },
                    };
                    SolveTableRow::new(&event, &solve, Some(rank), None)
                })
                .collect(),

            columns: LeaderboardTableColumns {
                event: false,
                rank: !self.history,
                solver: !self.history,
                record_holder: self.history,
                speed_cs: matches!(main_page_query, MainPageQuery::Speed { .. }),
                move_count: matches!(main_page_query, MainPageQuery::Fmc { .. }),
                date: true,
                program: true,
                total_solvers: false,
            },
        })
    }
}
