use std::collections::HashMap;

use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use itertools::Itertools;

use crate::db::{
    CategoryQuery, Event, FullSolve, MainPageCategory, ProgramQuery, ScoreQuery, VariantQuery,
};
use crate::traits::{Linkable, RequestBody};

#[derive(serde::Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum LeaderboardEvent {
    /// Single solve (speed)
    #[default]
    Single,
    /// Average (speed)
    Avg,
    /// Blindfolded (speed)
    Bld,
    /// One-handed (speed)
    Oh,
    /// Fewest-moves (FMC)
    Fmc,
    /// Computer-assisted fewest-moves (FMC)
    FmcCa,
    /// Distinct puzzles (aggregate)
    Distinct,
}

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

#[derive(serde::Serialize, Debug, Clone)]
pub struct SolveTableRow {
    pub rank: Option<i64>,

    pub puzzle_name: String,
    pub puzzle_url: String,
    pub uses_filters_icon: bool,
    pub uses_macros_icon: bool,
    pub uses_computer_assisted_icon: bool,
    pub allows_filters_icon: bool,
    pub allows_macros_icon: bool,
    pub allows_computer_assisted_icon: bool,

    pub solver_name: String,
    pub solver_url: String,

    pub speed_cs: Option<i32>,
    pub move_count: Option<i32>,
    pub solve_url: String,

    pub solve_date: DateTime<Utc>,

    pub program_abbreviation: String,

    pub total_solvers: Option<i64>,
}
impl SolveTableRow {
    pub fn new(
        event: &Event,
        solve: &FullSolve,
        rank: Option<i64>,
        total_solvers: Option<i64>,
        category_query: &CategoryQuery,
    ) -> Self {
        let puzzle_cat_q = match category_query {
            CategoryQuery::Speed {
                average,
                blind,
                filters,
                macros,
                one_handed,
                variant,
                program,
            } => {
                let default_material = match &solve.variant {
                    Some(v) => v.material_by_default,
                    None => false,
                };

                CategoryQuery::Speed {
                    average: *average,
                    blind: *blind,
                    filters: *filters,
                    macros: *macros,
                    one_handed: *one_handed,
                    variant: VariantQuery::from(&solve.variant),
                    program: match program {
                        ProgramQuery::All => {
                            if solve.program.material == default_material {
                                ProgramQuery::Default
                            } else {
                                match solve.program.material {
                                    true => ProgramQuery::Material,
                                    false => ProgramQuery::Virtual,
                                }
                            }
                        }
                        other => other.clone(),
                    },
                }
            }

            CategoryQuery::Fmc { .. } => category_query.clone(),
        };

        Self {
            rank,

            puzzle_name: event.name(),
            puzzle_url: event.puzzle.relative_url() + &puzzle_cat_q.url_query_params(true),
            uses_filters_icon: false,             // TODO
            uses_macros_icon: false,              // TODO
            uses_computer_assisted_icon: false,   // TODO
            allows_filters_icon: false,           // TODO
            allows_macros_icon: false,            // TODO
            allows_computer_assisted_icon: false, // TODO

            solver_name: solve.solver.display_name(),
            solver_url: solve.solver.relative_url() + &category_query.url_query_params(false),

            speed_cs: solve.speed_cs,
            move_count: solve.move_count,
            solve_url: solve.relative_url(),

            solve_date: solve.solve_date,

            program_abbreviation: solve.program.abbr.clone(),

            total_solvers,
        }
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct UserTableRow {
    rank: i64,

    solver_name: String,
    solver_url: String,

    score: String,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct LeaderboardTableResponse {
    pub table_rows: LeaderboardTableRows,
    pub columns: LeaderboardTableColumns,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum LeaderboardTableRows {
    Solves(Vec<SolveTableRow>),
    Users(Vec<UserTableRow>),
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct LeaderboardTableColumns {
    pub event: bool,
    pub rank: bool,
    pub solver: bool,
    pub record_holder: bool,
    pub speed_cs: bool,
    pub move_count: bool,
    pub date: bool,
    pub program: bool,
    pub total_solvers: bool,
    pub score: bool,
}

impl RequestBody for GlobalLeaderboardTable {
    type Response = LeaderboardTableResponse;

    async fn request(
        self,
        state: crate::AppState,
        _user: Option<crate::db::User>,
    ) -> Result<Self::Response, crate::error::AppError> {
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

                Ok(LeaderboardTableResponse {
                    table_rows: LeaderboardTableRows::Solves(rows),
                    columns: LeaderboardTableColumns {
                        event: true,
                        rank: false,
                        solver: false,
                        record_holder: true,
                        speed_cs: matches!(query, CategoryQuery::Speed { .. }),
                        move_count: matches!(query, CategoryQuery::Fmc { .. }),
                        date: true,
                        program: true,
                        total_solvers: true,
                        score: false,
                    },
                })
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

                Ok(LeaderboardTableResponse {
                    table_rows: LeaderboardTableRows::Users(rows),
                    columns: LeaderboardTableColumns {
                        event: false,
                        rank: true,
                        solver: true,
                        record_holder: false,
                        speed_cs: false,
                        move_count: false,
                        date: false,
                        program: false,
                        total_solvers: false,
                        score: true,
                    },
                })
            }
        }
    }
}

impl IntoResponse for LeaderboardTableResponse {
    fn into_response(self) -> axum::response::Response {
        crate::render_html_template(
            "solve-table-contents.html",
            &None,
            serde_json::to_value(&self).unwrap_or_default(),
        )
    }
}
