use crate::db::User;
use crate::db::UserPub;
use crate::error::AppError;
use crate::traits::RequestBody;
use crate::AppState;
use axum::response::Html;
use axum::response::IntoResponse;
use sqlx::query;

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
            /*"SELECT sol.*, UserAccount.display_name, UserAccount.dummy
            FROM (SELECT MIN(Solve.speed_cs) as speed_cs, Solve.user_id
                FROM Solve
                WHERE speed_cs IS NOT NULL
                AND Solve.puzzle_id = $1
                GROUP BY user_id) as sol
            JOIN UserAccount
            ON sol.user_id = UserAccount.id
            ORDER BY speed_cs
            ",*/
            "SELECT * FROM (SELECT DISTINCT ON (Solve.user_id)
                Solve.speed_cs, Solve.user_id, Solve.upload_time, Program.abbreviation, UserAccount.display_name, UserAccount.dummy
            FROM Solve
            JOIN UserAccount
            ON Solve.user_id = UserAccount.id
            JOIN ProgramVersion
            ON Solve.program_version_id = ProgramVersion.id
            JOIN Program
            ON ProgramVersion.program_id = Program.id
            WHERE speed_cs IS NOT NULL
            AND Solve.puzzle_id = $1
            ORDER BY Solve.user_id, Solve.speed_cs)
            ORDER BY speed_cs
            ",
            self.id
        )
        .fetch_all(&state.pool)
        .await?;

        let mut out = "".to_string();

        out += &puzzle_name;

        out += "<br>";
        out += "<table>";
        for (n, solve) in solves.into_iter().enumerate() {
            let user = UserPub {
                id: solve.user_id,
                display_name: solve.display_name,
                dummy: solve.dummy,
            };
            out += &format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                n + 1,
                user.html_name(),
                render_time(solve.speed_cs.expect("not null")),
                solve.upload_time.date_naive(),
                solve.abbreviation
            )
        }
        Ok(Html(out))
    }
}
