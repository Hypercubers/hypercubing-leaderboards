use axum::body::Body;
use axum::response::{IntoResponse, Response};

use super::LeaderboardEvent;
use super::global::{GlobalLeaderboardQuery, GlobalLeaderboardTable};
use crate::db::{
    Category, CategoryQuery, CombinedVariant, Event, ProgramQuery, Puzzle, PuzzleId,
    RankedFullSolve, User, VariantQuery,
};
use crate::html::solve_table::{
    LeaderboardTableColumns, LeaderboardTableRows, SolveTableRow, SolvesTable, SolvesTablesResponse,
};
use crate::traits::RequestBody;
use crate::{AppError, AppState};

#[derive(serde::Deserialize)]
pub struct PuzzleLeaderboard {
    id: PuzzleId,
}

pub struct PuzzleLeaderboardResponse {
    user: Option<User>,

    puzzle: Puzzle,
    variants: Vec<CombinedVariant>,
    history: bool,
}

impl RequestBody for PuzzleLeaderboard {
    type Response = PuzzleLeaderboardResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let puzzle = state.get_puzzle(self.id).await?.ok_or(AppError::NotFound)?;

        let variants = state.get_puzzle_combined_variants(puzzle.id).await?;

        Ok(PuzzleLeaderboardResponse {
            user,

            puzzle,
            variants,
            history: true,
        })
    }
}

impl IntoResponse for PuzzleLeaderboardResponse {
    fn into_response(self) -> Response<Body> {
        crate::render_html_template(
            "puzzle.html",
            &self.user,
            serde_json::json!({
                "puzzle": self.puzzle,
                "variants": self.variants,
                "history": self.history,
            }),
        )
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct PuzzleLeaderboardTable {
    pub id: PuzzleId,

    pub event: Option<LeaderboardEvent>,
    pub filters: Option<bool>,
    pub macros: Option<bool>,

    pub variant: Option<VariantQuery>,
    pub program: Option<ProgramQuery>,

    #[serde(default)]
    pub history: bool,
}

impl RequestBody for PuzzleLeaderboardTable {
    type Response = SolvesTablesResponse;

    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let global = GlobalLeaderboardTable {
            event: self.event,
            filters: self.filters,
            macros: self.macros,
            variant: Some(self.variant.unwrap_or(VariantQuery::Default)),
            program: Some(self.program.unwrap_or(ProgramQuery::Default)),
        };
        let GlobalLeaderboardQuery::Category(category_query) = global.global_leaderboard_query()
        else {
            return Err(AppError::InvalidQuery("bad category".to_string()));
        };
        let puzzle = state.get_puzzle(self.id).await?.ok_or(AppError::NotFound)?;

        let solves = if self.history {
            state
                .get_record_history(&puzzle, &category_query)
                .await?
                .into_iter()
                .map(|solve| RankedFullSolve { rank: 0, solve })
                .collect()
        } else {
            state
                .get_event_leaderboard(&puzzle, &category_query)
                .await?
        };

        let solve_rows = solves
            .into_iter()
            .map(|RankedFullSolve { rank, solve }| {
                let event = Event {
                    puzzle: puzzle.clone(),
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
                                None => puzzle.primary_filters,
                            };
                            let default_macros = match &solve.variant {
                                Some(v) => v.primary_macros,
                                None => puzzle.primary_macros,
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

        Ok(SolvesTable {
            heading: None,
            table_rows: LeaderboardTableRows::Solves(solve_rows),
            columns: LeaderboardTableColumns {
                puzzle: false,
                rank: !self.history,
                solver: !self.history,
                record_holder: self.history,
                speed_cs: matches!(category_query, CategoryQuery::Speed { .. }),
                move_count: matches!(category_query, CategoryQuery::Fmc { .. }),
                verified: false,
                date: true,
                program: true,
                total_solvers: false,
                score: false,
            },
        }
        .into())
    }
}
