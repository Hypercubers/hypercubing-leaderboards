use crate::db::puzzle::PuzzleCategory;
pub use crate::db::solve::LeaderboardSolve;
use crate::db::user::User;
use crate::error::AppError;
use crate::traits::RequestBody;
use crate::AppState;
use axum::body::Body;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;
use std::collections::HashMap;

pub fn render_time(time_cs: i32) -> String {
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

#[derive(serde::Deserialize, Clone)]
pub struct PuzzleLeaderboard {
    id: i32,
    blind: Option<String>,
    uses_filters: Option<bool>,
    uses_macros: Option<bool>,
}

pub struct PuzzleLeaderboardResponse {
    name: String,
    puzzle_category: PuzzleCategory,
    solves: Vec<LeaderboardSolve>,
}

impl RequestBody for PuzzleLeaderboard {
    type Response = PuzzleLeaderboardResponse;

    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let puzzle = state
            .get_puzzle(self.id)
            .await?
            .ok_or(AppError::InvalidQuery(format!(
                "Puzzle with id {} does not exist",
                self.id
            )))?;

        let blind = self.blind.is_some();
        let uses_filters = self.uses_filters.unwrap_or(puzzle.primary_filters);
        let uses_macros = self.uses_macros.unwrap_or(puzzle.primary_macros);
        let puzzle_category = PuzzleCategory {
            puzzle_id: self.id,
            blind,
            uses_filters,
            uses_macros,
        };

        let solves = state.get_leaderboard_puzzle(&puzzle_category).await?;

        Ok(PuzzleLeaderboardResponse {
            name: puzzle.name,
            puzzle_category,
            solves,
        })
    }
}

impl IntoResponse for PuzzleLeaderboardResponse {
    fn into_response(self) -> Response<Body> {
        let mut name = self.name.clone();
        let mut table_rows = "".to_string();

        name += &self.puzzle_category.format_modifiers();

        table_rows += &format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            "Rank", "Solver", "Time", "Date", "Program"
        );

        for (n, solve) in self.solves.into_iter().enumerate() {
            let url = format!("/solver?id={}", solve.user_id);
            table_rows += &format!(
                "<tr><td>{}</td><td><a href='{}'>{}</a></td><td>{}</td><td>{}</td><td>{}</td></tr>",
                n + 1,
                url,
                solve.user_html_name(),
                solve.speed_cs.map(render_time).unwrap_or("".to_string()),
                solve.upload_time.date_naive(),
                solve.abbreviation
            );
        }

        Html(format!(
            include_str!("../../html/puzzle.html"),
            name = name,
            table_rows = table_rows
        ))
        .into_response()
    }
}

#[derive(serde::Deserialize)]
pub struct SolverLeaderboard {
    id: i32,
}

pub struct SolverLeaderboardResponse {
    request: SolverLeaderboard,
    user: User,
    solves: Vec<(LeaderboardSolve, HashMap<PuzzleCategory, i32>)>,
}

impl RequestBody for SolverLeaderboard {
    type Response = SolverLeaderboardResponse;

    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let user = state
            .get_user(self.id)
            .await?
            .ok_or(AppError::InvalidQuery(format!(
                "Solver with id {} does not exist",
                self.id
            )))?;

        let mut solves = state.get_leaderboard_solver(self.id).await?;

        solves.sort_by_key(|solve| solve.puzzle_name.clone()); // don't need to clone?

        let mut solves_new = vec![];
        for solve in solves {
            let mut ranks = HashMap::new();
            for puzzle_category in solve.puzzle_category().supercategories() {
                let rank = state.get_rank(&puzzle_category, solve.speed_cs).await?;

                ranks.insert(puzzle_category, rank);
            }

            solves_new.push((solve, ranks))
        }

        Ok(SolverLeaderboardResponse {
            request: self,
            user,
            solves: solves_new,
        })
    }
}

impl IntoResponse for SolverLeaderboardResponse {
    fn into_response(self) -> Response<Body> {
        let name = self.user.html_name();
        let mut table_rows = "".to_string();

        table_rows += &format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            "Puzzle", "Rank", "Time", "Date", "Program"
        );

        for (solve, ranks) in self.solves {
            let url = format!(
                "puzzle?id={}{}&uses_filters={}&uses_macros={}",
                solve.puzzle_id,
                if solve.blind { "&blind" } else { "" },
                solve.uses_filters,
                solve.uses_macros
            );

            let puzzle_name =
                solve.puzzle_name.clone() + &solve.puzzle_category().format_modifiers();

            let mut rank_strs = vec![];
            for (puzzle_category, rank) in ranks.iter() {
                rank_strs.push(format!(
                    "{}{}{}",
                    rank,
                    if puzzle_category.uses_filters {
                        "⚗️"
                    } else {
                        ""
                    },
                    if puzzle_category.uses_macros {
                        "👾"
                    } else {
                        ""
                    },
                ))
            }

            let puzzle_category = solve.puzzle_category();
            let primary_category = puzzle_category.make_primary(solve.puzzle());
            let in_primary_category = primary_category >= puzzle_category;

            //dbg!(&ranks, &primary_category, &puzzle_category);
            let display_rank = if in_primary_category {
                ranks.get(&primary_category)
            } else {
                ranks.get(&puzzle_category)
            }
            .expect("must exist by partial order");

            table_rows += &format!(
                r#"<tr><td><a href='{}'>{}</td><td><span title="{}">{}{}</span></td><td>{}</td><td>{}</td><td>{}</td></tr>"#,
                url,
                puzzle_name,
                rank_strs.join("   |   "),
                display_rank,
                if in_primary_category { "" } else { "*" },
                solve.speed_cs.map(render_time).unwrap_or("".to_string()),
                solve.upload_time.date_naive(),
                solve.abbreviation
            );
        }

        Html(format!(
            include_str!("../../html/solver.html"),
            name = name,
            table_rows = table_rows
        ))
        .into_response()
    }
}
