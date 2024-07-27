use crate::db::puzzle::PuzzleCategory;
use crate::db::puzzle::PuzzleCategoryBase;
use crate::db::puzzle::PuzzleCategoryFlags;
pub use crate::db::solve::FullSolve;
use crate::db::user::User;
use crate::error::AppError;
use crate::traits::RequestBody;
use crate::AppState;
use axum::body::Body;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;
use std::collections::HashMap;

#[derive(serde::Deserialize, Clone)]
pub struct PuzzleLeaderboard {
    id: i32,
    blind: Option<String>,
    uses_filters: Option<bool>,
    uses_macros: Option<bool>,
}

pub struct PuzzleLeaderboardResponse {
    puzzle_category: PuzzleCategory,
    solves: Vec<FullSolve>,
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
        name += &self.puzzle_category.flags.format_modifiers();

        #[derive(serde::Serialize)]
        struct Row {
            rank: i32,
            user_url: String,
            user_name: String,
            solve_url: String,
            time: Option<i32>,
            date: String,
            abbreviation: String,
        }

        let mut table_rows = vec![];

        for (n, solve) in self.solves.into_iter().enumerate() {
            //   r#"<tr><td>{}</td><td><a href="{}">{}</a></td><td><a href="{}">{}</a></td><td>{}</td><td>{}</td></tr>"#
            table_rows.push(Row {
                rank: n as i32 + 1,
                user_url: solve.user().url_path(),
                user_name: solve.user().name(),
                solve_url: solve.url_path(),
                time: solve.speed_cs,
                date: solve.upload_time.date_naive().to_string(),
                abbreviation: solve.abbreviation,
            });
        }

        Html(
            crate::hbs!()
                .render(
                    "puzzle",
                    &serde_json::json!({
                        "name": name,
                        "table_rows": table_rows,
                    }),
                )
                .expect("render error"),
        )
        .into_response()
    }
}

#[derive(serde::Deserialize)]
pub struct SolverLeaderboard {
    id: i32,
}

pub struct SolverLeaderboardResponse {
    target_user: User,
    can_edit: bool,
    /// HashMap<puzzle id, HashMap<solve id, (FullSolve, Vec<PuzzleCategory>)>>
    solves: HashMap<PuzzleCategoryBase, HashMap<PuzzleCategoryFlags, (i32, FullSolve)>>,
}

impl RequestBody for SolverLeaderboard {
    type Response = SolverLeaderboardResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let target_user = state
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
                let rank = state.get_rank(&puzzle_category, &solve).await?;
                solves_new
                    .entry(puzzle_category.base.clone())
                    .or_insert(HashMap::new())
                    .entry(puzzle_category.flags.clone())
                    .and_modify(|e: &mut (i32, FullSolve)| {
                        if e.0 > rank {
                            *e = (rank, solve.clone());
                        }
                    })
                    .or_insert((rank, solve.clone()));
            }
        }

        let can_edit = target_user
            .to_public()
            .can_edit_opt(user.as_ref())
            .is_some();

        Ok(SolverLeaderboardResponse {
            target_user,
            can_edit,
            solves: solves_new,
        })
    }
}

impl IntoResponse for SolverLeaderboardResponse {
    fn into_response(self) -> Response<Body> {
        let name = self.target_user.to_public().name();

        #[derive(serde::Serialize)]
        struct Row {
            solve: FullSolve,
            has_primary: bool,
            puzzle_base_url: String,
            puzzle_base_name: String,
            puzzle_cat_url: String,
            flag_modifiers: String,
            rank: i32,
            solve_url: String,
        }

        let mut table_rows = vec![];
        let mut table_rows_non_primary = vec![];

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

            let has_primary = cat_map.contains_key(&puzzle_base.puzzle.primary_flags);
            let mut target_rows = vec![];

            let mut solve_map: Vec<_> = solve_map.into_iter().collect();
            solve_map.sort_by_key(|(f, _)| (Some(f) != primary_parent, f.order_key()));
            for (_, frs_vec) in solve_map.iter_mut() {
                frs_vec.sort_by_key(|(f, _, _)| f.order_key());
                for (flags, rank, solve) in frs_vec.iter() {
                    let puzzle_cat = PuzzleCategory {
                        base: puzzle_base.clone(),
                        flags: (*flags).clone(),
                    };

                    target_rows.push(Row {
                        solve: (*solve).clone(),
                        has_primary,
                        puzzle_base_url: puzzle_base.url_path(),
                        puzzle_base_name: puzzle_base.name(),
                        puzzle_cat_url: puzzle_cat.url_path(),
                        flag_modifiers: flags.format_modifiers(),
                        rank: **rank,
                        solve_url: solve.url_path(),
                    });
                }
            }

            if has_primary {
                table_rows.push(target_rows)
            } else {
                table_rows_non_primary.push(target_rows)
            }
        }

        table_rows.extend(table_rows_non_primary);

        Html(
            crate::hbs!()
                .render(
                    "solver",
                    &serde_json::json!({
                        "user_id": self.target_user.id,
                        "name": name,
                        "can_edit": self.can_edit,
                        "table_rows": table_rows,
                    }),
                )
                .expect("render error"),
        )
        .into_response()
    }
}

#[derive(serde::Deserialize)]
pub struct GlobalLeaderboard {}

pub struct GlobalLeaderboardResponse {
    solves: HashMap<PuzzleCategoryBase, HashMap<PuzzleCategoryFlags, FullSolve>>,
    total_solvers_map: HashMap<PuzzleCategoryBase, HashMap<PuzzleCategoryFlags, i32>>,
}

impl RequestBody for GlobalLeaderboard {
    type Response = GlobalLeaderboardResponse;

    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let solves = state.get_leaderboard_global().await?;

        // solves.sort_by_key(|solve| solve.puzzle_name.clone()); // don't need to clone?

        let mut solves_new = HashMap::new();
        let mut total_solvers_map = HashMap::new();
        for solve in solves {
            for puzzle_category in solve.puzzle_category().supercategories() {
                solves_new
                    .entry(puzzle_category.base.clone())
                    .or_insert(HashMap::new())
                    .entry(puzzle_category.flags.clone())
                    .and_modify(|e: &mut FullSolve| {
                        if solve.rank_key() < e.rank_key() {
                            *e = solve.clone();
                        }
                    })
                    .or_insert(solve.clone());

                let total_solvers_submap = total_solvers_map
                    .entry(puzzle_category.base.clone())
                    .or_insert(HashMap::new());
                if !total_solvers_submap.contains_key(&puzzle_category.flags) {
                    let total_solvers =
                        state.get_leaderboard_puzzle(&puzzle_category).await?.len() as i32;
                    total_solvers_submap.insert(puzzle_category.flags, total_solvers);
                }
            }
        }

        Ok(GlobalLeaderboardResponse {
            solves: solves_new,
            total_solvers_map,
        })
    }
}

impl IntoResponse for GlobalLeaderboardResponse {
    fn into_response(self) -> Response<Body> {
        #[derive(serde::Serialize)]
        struct Row {
            solve: FullSolve,
            puzzle_base_url: String,
            puzzle_base_name: String,
            puzzle_cat_url: String,
            flag_modifiers: String,
            user_url: String,
            user_name: String,
            solve_url: String,
            total_solvers: i32,
        }

        let mut table_rows = vec![];

        let mut solves: Vec<_> = self.solves.into_iter().collect();
        solves.sort_by_key(|(p, _)| p.puzzle.name.clone());
        for (puzzle_base, cat_map) in solves {
            let mut target_rows = vec![];
            let mut solve_map: Vec<_> = cat_map.into_iter().collect();
            solve_map.sort_by_key(|(f, _)| (*f != puzzle_base.puzzle.primary_flags, f.order_key()));

            for (flags, solve) in solve_map.iter_mut() {
                let puzzle_cat = PuzzleCategory {
                    base: puzzle_base.clone(),
                    flags: (*flags).clone(),
                };

                target_rows.push(Row {
                    solve: (*solve).clone(),
                    puzzle_base_url: puzzle_base.url_path(),
                    puzzle_base_name: puzzle_base.name(),
                    puzzle_cat_url: puzzle_cat.url_path(),
                    flag_modifiers: flags.format_modifiers(),
                    user_url: solve.user().url_path(),
                    user_name: solve.user().name(),
                    solve_url: solve.url_path(),
                    total_solvers: self.total_solvers_map[&puzzle_base][flags],
                });
            }

            target_rows[1..].sort_by_key(|r| -r.total_solvers);
            table_rows.push(target_rows);
        }

        table_rows.sort_by_key(|rr| -rr[0].total_solvers);

        Html(
            crate::hbs!()
                .render(
                    "index",
                    &serde_json::json!({
                        "table_rows": table_rows,
                    }),
                )
                .expect("render error"),
        )
        .into_response()
    }
}
