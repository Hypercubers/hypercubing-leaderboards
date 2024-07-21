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
        let mut name = self.puzzle_category.base.name();
        let mut table_rows = "".to_string();

        name += &self.puzzle_category.flags.format_modifiers();

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
            user,
            solves: solves_new,
        })
    }
}

impl IntoResponse for SolverLeaderboardResponse {
    fn into_response(self) -> Response<Body> {
        let name = self.user.html_name();
        let mut table_rows = "".to_string();
        let mut table_rows_non_primary = "".to_string();

        let mut solves: Vec<_> = self.solves.into_iter().collect();
        solves.sort_by_key(|(p, _)| p.puzzle.name.clone());
        for (puzzle_base, cat_map) in solves {
            let mut solve_map = HashMap::new();
            let mut primary_parent = None;
            for (flags, (rank, solve)) in &cat_map {
                solve_map
                    .entry(solve.puzzle_category().flags)
                    .or_insert(vec![])
                    .push((flags, rank, solve));

                if *flags == puzzle_base.puzzle.primary_flags {
                    primary_parent = Some(flags)
                }
            }

            let get_primary = cat_map.get(&puzzle_base.puzzle.primary_flags);
            let target_rows;
            if get_primary.is_some() {
                target_rows = &mut table_rows;
            } else {
                target_rows = &mut table_rows_non_primary;
            }

            let row_tbody_header = r#"<tbody class="hide-subcategories"><tr>
                <td><input type="checkbox" class="expand-subcategories"/></td>"#;
            let mut has_header = false;
            let mut skip_next_row = false;

            if let Some((rank, solve)) = get_primary {
                let url = format!(
                    "puzzle?id={}{}",
                    solve.puzzle_id,
                    if solve.blind { "&blind" } else { "" },
                );

                *target_rows += row_tbody_header;
                *target_rows += &format!(
                    r#"<td><a href='{}'>{}<span class="subcategory-row">: {}</span></td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>"#,
                    url,
                    puzzle_base.name(),
                    puzzle_base.puzzle.primary_flags.format_modifiers(),
                    rank,
                    solve.speed_cs.map(render_time).unwrap_or("".to_string()),
                    solve.upload_time.date_naive(),
                    solve.abbreviation
                );
                has_header = true;
                skip_next_row = true;
            }

            let mut solve_map: Vec<_> = solve_map.into_iter().collect();
            solve_map.sort_by_key(|(f, _)| (Some(f) != primary_parent, f.order_key()));
            for (_, frs_vec) in solve_map.iter_mut() {
                frs_vec.sort_by_key(|(f, _, _)| f.order_key());
                for (j, (flags, rank, solve)) in frs_vec.iter().enumerate() {
                    let url = format!(
                        "puzzle?id={}{}{}",
                        solve.puzzle_id,
                        if solve.blind { "&blind" } else { "" },
                        flags.url_params()
                    );

                    if skip_next_row {
                        // this row has already been added
                        skip_next_row = false;
                    } else if j == 0 {
                        if has_header {
                            *target_rows += r#"<tr class="subcategory-row"><td></td>"#;
                        } else {
                            *target_rows += row_tbody_header;
                        }

                        *target_rows += &format!(
                            r#"<td><a href='{}'>{}: {}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>"#,
                            url,
                            puzzle_base.name(),
                            flags.format_modifiers(),
                            rank,
                            solve.speed_cs.map(render_time).unwrap_or("".to_string()),
                            solve.upload_time.date_naive(),
                            solve.abbreviation
                        );
                    } else {
                        *target_rows += &format!(
                            r#"<tr class="subcategory-row"><td></td><td><a href='{}'>{}: {}</td><td>{}</td><td></td><td></td><td></td></tr>"#,
                            url,
                            solve.puzzle_name,
                            flags.format_modifiers(),
                            rank,
                        );
                    }

                    has_header = true;
                }
            }

            *target_rows += "</tbody>"
        }

        Html(format!(
            include_str!("../../html/solver.html"),
            name = name,
            table_rows = table_rows,
            table_rows_non_primary = table_rows_non_primary,
        ))
        .into_response()
    }
}
