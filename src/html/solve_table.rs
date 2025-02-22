use axum::response::IntoResponse;
use chrono::{DateTime, Utc};

use crate::{
    db::FullSolve,
    traits::{Linkable, RequestBody},
};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct AllPuzzlesLeaderboard {
    event: Option<LeaderboardEvent>,
    blind: Option<bool>,
    filters: Option<bool>,
    macros: Option<bool>,
    computer: Option<bool>,
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

    upload_date: DateTime<Utc>,

    program_abbreviation: String,

    total_solvers: Option<i64>,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct AllPuzzlesLeaderboardResponse {
    table_rows: Vec<SolveTableRow>,

    show_solver_column: bool,
    show_record_holder_column: bool,
}

#[derive(serde::Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
enum LeaderboardEvent {
    #[default]
    Speed,
    Fmc,
}

impl RequestBody for AllPuzzlesLeaderboard {
    type Response = AllPuzzlesLeaderboardResponse;

    async fn request(
        self,
        state: crate::AppState,
        _user: Option<crate::db::User>,
    ) -> Result<Self::Response, crate::error::AppError> {
        let event = self.event.unwrap_or_default();

        let solves = match event {
            LeaderboardEvent::Speed => {
                // state
                //     .get_speed_records(self.blind.unwrap_or(false), self.filters, self.macros)
                //     .await?
            }
            LeaderboardEvent::Fmc => {
                // state
                //     .get_fmc_records(self.computer.unwrap_or(false))
                //     .await?
            }
        };

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

        // Ok(AllPuzzlesLeaderboardResponse {
        //     table_rows: rows,
        //     show_solver_column: false,
        //     show_record_holder_column: true,
        // })

        todo!()
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
