use crate::db::User;
use crate::db::UserPub;
use crate::error::AppError;
use crate::traits::RequestBody;
use crate::AppState;
use axum::response::Html;
use axum::response::IntoResponse;
use sqlx::{query, query_as};

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

#[derive(serde::Deserialize)]
pub struct PuzzleLeaderboard {
    id: i32,
    blind: Option<String>,
    no_filters: Option<String>,
    no_macros: Option<String>,
}

impl RequestBody for PuzzleLeaderboard {
    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
        _log_file: Option<String>,
    ) -> Result<impl IntoResponse, AppError> {
        let puzzle_name = query!(
            "SELECT name
            FROM Puzzle
            WHERE id = $1",
            self.id
        )
        .fetch_optional(&state.pool)
        .await?
        .ok_or(AppError::InvalidQuery(format!(
            "Puzzle with id {} does not exist",
            self.id
        )))?
        .name;

        let solves = query!(
            "SELECT * FROM (SELECT DISTINCT ON (Solve.user_id)
                Solve.speed_cs, Solve.user_id, Solve.upload_time, Program.abbreviation, UserAccount.display_name, UserAccount.dummy
                FROM Solve
                JOIN UserAccount ON Solve.user_id = UserAccount.id
                JOIN ProgramVersion ON Solve.program_version_id = ProgramVersion.id
                JOIN Program ON ProgramVersion.program_id = Program.id
                JOIN Puzzle ON Solve.puzzle_id = Puzzle.id
                WHERE speed_cs IS NOT NULL
                    AND Puzzle.leaderboard = $1
                    AND Solve.blind = $2
                    AND (NOT (Solve.uses_filters AND $3))
                    AND (NOT (Solve.uses_macros AND $4))
                ORDER BY Solve.user_id, Solve.speed_cs ASC)
            ORDER BY speed_cs ASC
            ",
            self.id,
            self.blind.is_some(),
            self.no_filters.is_some(),
            self.no_macros.is_some()
        )
        .fetch_all(&state.pool)
        .await?;

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
            let user = UserPub {
                id: solve.user_id,
                display_name: solve.display_name,
                dummy: solve.dummy,
            };
            let url = format!("/solver?id={}", user.id);
            out += &format!(
                "<tr><td>{}</td><td><a href='{}'>{}</a></td><td>{}</td><td>{}</td><td>{}</td></tr>",
                n + 1,
                url,
                user.html_name(),
                render_time(solve.speed_cs.expect("not null")),
                solve.upload_time.date_naive(),
                solve.abbreviation
            );
        }
        Ok(Html(out))
    }
}

#[derive(serde::Deserialize)]
pub struct SolverLeaderboard {
    id: i32,
}

impl RequestBody for SolverLeaderboard {
    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
        _log_file: Option<String>,
    ) -> Result<impl IntoResponse, AppError> {
        let user = query_as!(
            UserPub,
            "SELECT id, display_name, dummy
            FROM UserAccount
            WHERE id = $1",
            self.id
        )
        .fetch_optional(&state.pool)
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
                    let solves = query!(
                        "SELECT DISTINCT ON (Puzzle.leaderboard)
                            Solve.speed_cs, Solve.user_id, Solve.upload_time, Program.abbreviation, Solve.puzzle_id, Puzzle.name, Solve.uses_filters, Solve.uses_macros, Puzzle.leaderboard
                        FROM Solve
                        JOIN Puzzle ON Solve.puzzle_id = Puzzle.id
                        JOIN ProgramVersion ON Solve.program_version_id = ProgramVersion.id
                        JOIN Program ON ProgramVersion.program_id = Program.id
                        WHERE speed_cs IS NOT NULL
                            AND Solve.user_id = $1
                            AND Solve.blind = $2
                            AND (NOT (Solve.uses_filters AND $3))
                            AND (NOT (Solve.uses_macros AND $4))
                            AND Puzzle.leaderboard IS NOT NULL
                        ORDER BY Puzzle.leaderboard, Solve.speed_cs ASC
                        ",
                        self.id,
                        blind,
                        no_filters,
                        no_macros
                    )
                    .fetch_all(&state.pool)
                    .await?;

                    for solve in solves {
                        if solve.uses_filters == !no_filters && solve.uses_macros == !no_macros {
                            let rank = query!(
                                "SELECT COUNT(*) FROM (SELECT DISTINCT ON (user_id)
                                        user_id
                                    FROM Solve
                                    JOIN Puzzle ON Solve.puzzle_id = Puzzle.id
                                    WHERE speed_cs IS NOT NULL
                                        AND Puzzle.leaderboard = $1
                                        AND blind = $2
                                        AND (NOT (uses_filters AND $3))
                                        AND (NOT (uses_macros AND $4))
                                        AND speed_cs < $5
                                    ORDER BY user_id, speed_cs ASC)
                                    ",
                                solve.puzzle_id,
                                blind,
                                no_filters,
                                no_macros,
                                solve.speed_cs
                            )
                            .fetch_one(&state.pool)
                            .await?
                            .count
                            .expect("count should not be null")
                                + 1;

                            let puzzle_name = format!(
                                "{}{}{}{}",
                                solve.name,
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

        Ok(Html(out))
    }
}
