use std::collections::HashMap;

use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use itertools::Itertools;

use crate::db::{MainPageCategory, MainPageQuery};
use crate::traits::{Linkable, RequestBody};

#[derive(serde::Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum LeaderboardEvent {
    #[default]
    Single,
    /// Average
    Avg,
    /// Blindfolded
    Bld,
    /// One-handed
    Oh,
    /// Fewest-moves
    Fmc,
    /// Computer-assisted fewest-moves
    FmcCa,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct AllPuzzlesLeaderboard {
    event: Option<LeaderboardEvent>,
    filters: Option<bool>,
    macros: Option<bool>,
}

#[derive(serde::Serialize, Debug, Clone)]
struct SolveTableRow {
    puzzle_name: String,
    puzzle_category_url: String,
    uses_filters_icon: bool,
    uses_macros_icon: bool,
    uses_computer_assisted_icon: bool,
    allows_filters_icon: bool,
    allows_macros_icon: bool,
    allows_computer_assisted_icon: bool,

    user_name: String,
    user_url: String,

    speed_cs: Option<i32>,
    move_count: Option<i32>,

    solve_date: DateTime<Utc>,

    program_abbreviation: String,

    total_solvers: i64,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct AllPuzzlesLeaderboardResponse {
    table_rows: Vec<SolveTableRow>,

    show_solver_column: bool,
    show_record_holder_column: bool,
}

impl RequestBody for AllPuzzlesLeaderboard {
    type Response = AllPuzzlesLeaderboardResponse;

    async fn request(
        self,
        state: crate::AppState,
        _user: Option<crate::db::User>,
    ) -> Result<Self::Response, crate::error::AppError> {
        let event = self.event.unwrap_or_default();
        let is_fmc = matches!(event, LeaderboardEvent::Fmc | LeaderboardEvent::FmcCa);
        let is_speed = !is_fmc;

        let query = if is_speed {
            MainPageQuery::Speed {
                average: event == LeaderboardEvent::Avg,
                blind: event == LeaderboardEvent::Bld,
                filters: self.filters,
                macros: self.macros,
                one_handed: event == LeaderboardEvent::Oh,
            }
        } else {
            MainPageQuery::Fmc {
                computer_assisted: event == LeaderboardEvent::FmcCa,
            }
        };

        let solver_counts: HashMap<MainPageCategory, i64> = state
            .get_all_puzzles_counts(query)
            .await?
            .into_iter()
            .collect();

        let solves = state.get_all_puzzles_leaderboard(query).await?;

        let rows = solves
            .iter()
            .map(|(solve_event, solve)| {
                SolveTableRow {
                    puzzle_name: solve_event.name(),
                    puzzle_category_url: solve_event.relative_url(),
                    uses_filters_icon: false,             // TODO
                    uses_macros_icon: false,              // TODO
                    uses_computer_assisted_icon: false,   // TODO
                    allows_filters_icon: false,           // TODO
                    allows_macros_icon: false,            // TODO
                    allows_computer_assisted_icon: false, // TODO
                    user_name: solve.solver.display_name(),
                    user_url: solve.solver.relative_url(),
                    speed_cs: solve.speed_cs,
                    move_count: solve.move_count,
                    solve_date: solve.solve_date,
                    program_abbreviation: solve.program.abbr.clone(),
                    total_solvers: *solver_counts
                        .get(&if is_speed {
                            MainPageCategory::StandardSpeed {
                                puzzle: solve.puzzle.id,
                                variant: solve.variant.as_ref().map(|v| v.id),
                                material: solve.program.material,
                            }
                        } else {
                            MainPageCategory::StandardFmc {
                                puzzle: solve.puzzle.id,
                            }
                        })
                        .unwrap_or(&0),
                }
            })
            .sorted_by_key(|row| -row.total_solvers)
            .collect();

        // let rows = solves
        //     .into_iter()
        //     .map(|(solve, total_solvers)| SolveTableRow {
        //         puzzle_name: solve.puzzle().name.clone(),
        //         puzzle_category_url: match event {
        //             LeaderboardEvent::Speed => solve.category.speed_relative_url(),
        //             LeaderboardEvent::Fmc => solve.category.fmc_relative_url(),
        //         },
        //         uses_filters_icon: solve.flags().uses_filters,
        //         uses_macros_icon: solve.flags().uses_macros,
        //         uses_computer_assisted_icon: solve.flags().computer_assisted,
        //         allows_filters_icon: self.filters.is_none()
        //             && solve.puzzle().primary_flags.uses_filters,
        //         allows_macros_icon: self.macros.is_none()
        //             && solve.puzzle().primary_flags.uses_macros,
        //         allows_computer_assisted_icon: self.computer.is_none()
        //             && solve.puzzle().primary_flags.computer_assisted,
        //         user_name: solve.user.name(),
        //         user_url: solve.user.relative_url(),
        //         speed_cs: solve.speed_cs,
        //         move_count: solve.move_count,
        //         upload_date: solve.upload_date,
        //         program_abbreviation: solve.program_version.abbreviation(),
        //         total_solvers: Some(total_solvers),
        //     })
        //     .collect();

        Ok(AllPuzzlesLeaderboardResponse {
            table_rows: rows,
            show_solver_column: false,
            show_record_holder_column: true,
        })
    }
}

impl IntoResponse for AllPuzzlesLeaderboardResponse {
    fn into_response(self) -> axum::response::Response {
        crate::render_html_template(
            "solve-table.html",
            &None,
            serde_json::to_value(&self).unwrap_or_default(),
        )
    }
}
