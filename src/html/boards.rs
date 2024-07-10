pub use crate::db::solve::{LeaderboardSolve, PuzzleLeaderboard};
use crate::db::user::User;
use crate::error::AppError;
use crate::traits::{RequestBody, RequestResponse};
use crate::AppState;
use axum::response::Html;
use axum::response::IntoResponse;

fn render_time(time_cs: i32) -> String {
    let cs = time_cs % 100;
    let s = (time_cs / 100) % 60;
    let m = (time_cs / (100 * 60)) % 60;
    let h = (time_cs / (100 * 60 * 60)) % 24;
    let d = time_cs / (100 * 60 * 60 * 24);

    if d > 0 {
        format!("{d}:{h:0>2}:{m:0>2}:{s:0>2}.{cs:0>2}")
    } else if h > 0 {
        format!("{h}:{m:0>2}:{s:0>2}.{cs:0>2}")
    } else if m > 0 {
        format!("{m}:{s:0>2}.{cs:0>2}")
    } else {
        format!("{s}.{cs:0>2}")
    }
}

struct PuzzleLeaderboardResponse {
    out: String,
}

impl RequestBody for PuzzleLeaderboard {
    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<impl RequestResponse, AppError> {
        let puzzle_name = state
            .get_puzzle(self.id)
            .await?
            .ok_or(AppError::InvalidQuery(format!(
                "Puzzle with id {} does not exist",
                self.id
            )))?
            .name;

        let mut solves = state.get_leaderboard_puzzle(self.clone()).await?;

        solves.sort_by_key(|solve| (solve.speed_cs, solve.upload_time));
        let solves = solves;

        let mut out = "".to_string();

        out += &puzzle_name;

        if self.blind.is_some() {
            out += "🙈";
        }
        if self.no_filters.is_none() {
            out += "⚗️";
        }
        if self.no_macros.is_none() {
            out += "👾";
        }

        out += "<br>";
        out += "<table>";

        out += &format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            "Rank", "Solver", "Time", "Date", "Program"
        );

        for (n, solve) in solves.into_iter().enumerate() {
            let url = format!("/solver?id={}", solve.user_id);
            out += &format!(
                "<tr><td>{}</td><td><a href='{}'>{}</a></td><td>{}</td><td>{}</td><td>{}</td></tr>",
                n + 1,
                url,
                solve.user_html_name(),
                render_time(solve.speed_cs.expect("not null")),
                solve.upload_time.date_naive(),
                solve.abbreviation
            );
        }
        Ok(PuzzleLeaderboardResponse { out })
    }
}

impl RequestResponse for PuzzleLeaderboardResponse {
    async fn as_axum_response(self) -> impl IntoResponse {
        Html(self.out)
    }
}

#[derive(serde::Deserialize)]
pub struct SolverLeaderboard {
    id: i32,
}

struct SolverLeaderboardResponse {
    out: String,
}

impl RequestBody for SolverLeaderboard {
    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<impl RequestResponse, AppError> {
        let user = state
            .get_user(self.id)
            .await?
            .ok_or(AppError::InvalidQuery(format!(
                "Solver with id {} does not exist",
                self.id
            )))?;

        let mut out = "".to_string();

        out += &user.html_name();

        out += "<br>";
        out += "<table>";

        out += &format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            "Puzzle", "Rank", "Time", "Date", "Program"
        );

        for blind in [false, true] {
            for no_filters in [false, true] {
                for no_macros in [false, true] {
                    let mut solves = state
                        .get_leaderboard_solver(self.id, blind, no_filters, no_macros)
                        .await?;

                    solves.sort_by_key(|solve| solve.puzzle_name.clone()); // don't need to clone?
                    let solves = solves;

                    for solve in solves {
                        if solve.uses_filters == !no_filters && solve.uses_macros == !no_macros {
                            let rank = state
                                .get_rank(
                                    solve.puzzle_id,
                                    blind,
                                    no_filters,
                                    no_macros,
                                    solve.speed_cs.expect("should exist"),
                                )
                                .await?;

                            let puzzle_name = format!(
                                "{}{}{}{}",
                                solve.puzzle_name,
                                if blind { "🙈" } else { "" },
                                if no_filters { "" } else { "⚗️" },
                                if no_macros { "" } else { "👾" },
                            );

                            let url = format!(
                                "puzzle?id={}{}{}{}",
                                solve.leaderboard.expect("not null"),
                                if blind { "&blind" } else { "" },
                                if no_filters { "&no_filters" } else { "" },
                                if no_macros { "&no_macros" } else { "" }
                            );

                            out += &format!(
                                "<tr><td><a href='{}'>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                                url,puzzle_name,
                                rank,
                                render_time(solve.speed_cs.expect("not null")),
                                solve.upload_time.date_naive(),
                                solve.abbreviation
                            );
                        }
                    }
                }
            }
        }

        Ok(SolverLeaderboardResponse { out })
    }
}

impl RequestResponse for SolverLeaderboardResponse {
    async fn as_axum_response(self) -> impl IntoResponse {
        Html(self.out)
    }
}
