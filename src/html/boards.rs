use crate::db::puzzle::Puzzle;
use crate::db::puzzle::PuzzleCategory;
use crate::db::puzzle::PuzzleCategoryBase;
use crate::db::puzzle::PuzzleCategoryFlags;
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
        let uses_filters = self
            .uses_filters
            .unwrap_or(puzzle.primary_flags.uses_filters);
        let uses_macros = self.uses_macros.unwrap_or(puzzle.primary_flags.uses_macros);
        let puzzle_category = PuzzleCategory {
            base: PuzzleCategoryBase { puzzle, blind },
            flags: PuzzleCategoryFlags {
                uses_filters,
                uses_macros,
            },
        };

        let solves = state.get_leaderboard_puzzle(&puzzle_category).await?;

        Ok(PuzzleLeaderboardResponse {
            puzzle_category,
            solves,
        })
    }
}

impl IntoResponse for PuzzleLeaderboardResponse {
    fn into_response(self) -> Response<Body> {
        let mut name = self.puzzle_category.base.puzzle.name.clone();
        let mut table_rows = "".to_string();

        if self.puzzle_category.base.blind {
            name += " Blind"
        }
        name += &self.puzzle_category.flags.format_modifiers();

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
    /// HashMap<puzzle id, HashMap<solve id, (LeaderboardSolve, Vec<PuzzleCategory>)>>
    solves: HashMap<PuzzleCategoryBase, HashMap<PuzzleCategoryFlags, (i32, LeaderboardSolve)>>,
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

        let mut solves_new = HashMap::new();
        for solve in solves {
            for puzzle_category in solve.puzzle_category().supercategories() {
                let rank = state.get_rank(&puzzle_category, solve.speed_cs).await?;
                solves_new
                    .entry(puzzle_category.base.clone())
                    .or_insert(HashMap::new())
                    .entry(puzzle_category.flags.clone())
                    .and_modify(|e: &mut (i32, LeaderboardSolve)| {
                        if e.0 > rank {
                            *e = (rank, solve.clone());
                        }
                    })
                    .or_insert((rank, solve.clone()));
            }
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

        let mut solves: Vec<_> = self.solves.into_iter().collect();
        solves.sort_by_key(|(p, _)| p.puzzle.name.clone());
        for (puzzle_base, cat_map) in solves {
            if let Some((rank, solve)) = cat_map.get(&puzzle_base.puzzle.primary_flags) {
                let url = format!(
                    "puzzle?id={}{}",
                    solve.puzzle_id,
                    if solve.blind { "&blind" } else { "" },
                );

                table_rows += &format!(
                    r#"<tr><td><a href='{}'>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>"#,
                    url,
                    solve.puzzle_name,
                    rank,
                    solve.speed_cs.map(render_time).unwrap_or("".to_string()),
                    solve.upload_time.date_naive(),
                    solve.abbreviation
                );
            }
        }

        Html(format!(
            include_str!("../../html/solver.html"),
            name = name,
            table_rows = table_rows
        ))
        .into_response()
    }
}
