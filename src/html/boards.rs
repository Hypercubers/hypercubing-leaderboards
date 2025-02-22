use std::collections::HashMap;

use axum::body::Body;
use axum::response::{IntoResponse, Response};

pub use crate::db::FullSolve;
use crate::db::PuzzleId;
use crate::db::RankedFullSolve;
use crate::db::{User, UserId};
use crate::error::AppError;
use crate::traits::{Linkable, RequestBody};
use crate::AppState;

#[derive(serde::Deserialize, Clone)]
pub struct PuzzleLeaderboard {
    id: PuzzleId,
    blind: Option<String>,
    uses_filters: Option<bool>,
    uses_macros: Option<bool>,
    computer_assisted: Option<bool>,
}

pub struct PuzzleLeaderboardResponse {
    // puzzle_category: PuzzleCategory,
    solves: Vec<RankedFullSolve>,
    user: Option<User>,
}

impl RequestBody for PuzzleLeaderboard {
    type Response = PuzzleLeaderboardResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let puzzle = state
            .get_puzzle(self.id)
            .await?
            .ok_or(AppError::InvalidQuery(format!(
                "Puzzle with id {} does not exist",
                self.id.0
            )))?;

        let blind = self.blind.is_some();
        let uses_filters = self.uses_filters.unwrap_or(puzzle.primary_filters);
        let uses_macros = self.uses_macros.unwrap_or(puzzle.primary_macros);
        let computer_assisted = self.computer_assisted.unwrap_or(false);
        // let puzzle_category = PuzzleCategory {
        //     base: PuzzleCategoryBase { puzzle, blind },
        //     flags: PuzzleCategoryFlags {
        //         uses_filters,
        //         uses_macros,
        //         computer_assisted,
        //     },
        // };

        // let solves = state.get_puzzle_speed_leaderboard(&puzzle_category).await?;

        // Ok(PuzzleLeaderboardResponse {
        //     puzzle_category,
        //     solves,
        //     user,
        // })

        todo!()
    }
}

impl IntoResponse for PuzzleLeaderboardResponse {
    fn into_response(self) -> Response<Body> {
        // let mut name = self.puzzle_category.base.name();
        // name += &self.puzzle_category.flags.emoji_string();

        // #[derive(serde::Serialize)]
        // struct Row {
        //     rank: i32,
        //     user_url: String,
        //     user_name: String,
        //     solve_url: String,
        //     time: Option<i32>,
        //     date: String,
        //     abbreviation: String,
        // }

        // let mut table_rows = vec![];

        // for solve in self.solves {
        //     table_rows.push(Row {
        //         rank: solve.rank as i32,
        //         user_url: solve.solve.user.relative_url(),
        //         user_name: solve.solve.user.name(),
        //         solve_url: solve.solve.url_path(),
        //         time: solve.solve.speed_cs,
        //         date: solve.solve.upload_date.date_naive().to_string(),
        //         abbreviation: solve.solve.program_version.abbreviation(),
        //     });
        // }

        // crate::render_html_template(
        //     "puzzle.html",
        //     &self.user,
        //     serde_json::json!({
        //         "name": name,
        //         "table_rows": table_rows,
        //     }),
        // )

        todo!()
    }
}

#[derive(serde::Deserialize)]
pub struct SolverLeaderboard {
    id: UserId,
}

pub struct SolverLeaderboardResponse {
    target_user: User,
    can_edit: bool,
    /// `HashMap<puzzle id, HashMap<solve id, (FullSolve, Vec<PuzzleCategory>)>>`
    // solves: HashMap<PuzzleCategoryBase, HashMap<PuzzleCategoryFlags, (i64, FullSolve)>>,
    user: Option<User>,
}

impl RequestBody for SolverLeaderboard {
    type Response = SolverLeaderboardResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        // let target_user = state
        //     .get_user(self.id)
        //     .await?
        //     .ok_or(AppError::InvalidQuery(format!(
        //         "Solver with id {} does not exist",
        //         self.id.0
        //     )))?;

        // let mut solves = state.get_solver_speed_pbs(self.id).await?;

        // solves.sort_by_key(|solve| solve.solve.puzzle().name.clone()); // TODO: avoid clone?

        // let mut solves_new = HashMap::new();
        // for solve in solves {
        //     let RankedFullSolve { rank, solve } = solve;
        //     for puzzle_category in solve.category.speed_supercategories() {
        //         solves_new
        //             .entry(puzzle_category.base.clone())
        //             .or_insert(HashMap::new())
        //             .entry(puzzle_category.flags)
        //             .and_modify(|e: &mut (i64, FullSolve)| {
        //                 if e.0 > rank {
        //                     *e = (rank, solve.clone());
        //                 }
        //             })
        //             .or_insert((rank, solve.clone()));
        //     }
        // }

        // let can_edit = target_user
        //     .to_public()
        //     .can_edit_opt(user.as_ref())
        //     .is_some();

        // Ok(SolverLeaderboardResponse {
        //     target_user,
        //     can_edit,
        //     solves: solves_new,
        //     user,
        // })

        todo!()
    }
}

impl IntoResponse for SolverLeaderboardResponse {
    fn into_response(self) -> Response<Body> {
        // let name = self.target_user.to_public().name();

        // #[derive(serde::Serialize)]
        // struct Row {
        //     solve: FullSolve,
        //     has_primary: bool,
        //     puzzle_base_url: String,
        //     puzzle_base_name: String,
        //     puzzle_cat_url: String,
        //     flag_modifiers: String,
        //     rank: i32,
        //     solve_url: String,
        // }

        // let mut speedsolves = vec![];
        // let mut speedsolves_non_primary = vec![];

        // let mut solves: Vec<_> = self.solves.into_iter().collect();
        // solves.sort_by_key(|(p, _)| p.puzzle.name.clone());
        // for (puzzle_base, cat_map) in solves {
        //     let mut solve_map = HashMap::new();
        //     let mut primary_parent = None;
        //     for (flags, (rank, solve)) in &cat_map {
        //         solve_map
        //             .entry(solve.category.flags)
        //             .or_insert(vec![])
        //             .push((flags, rank, solve));

        //         if *flags == puzzle_base.puzzle.primary_flags {
        //             primary_parent = Some(flags);
        //         }
        //     }

        //     let has_primary = cat_map.contains_key(&puzzle_base.puzzle.primary_flags);
        //     let mut target_rows = vec![];

        //     let mut solve_map: Vec<_> = solve_map.into_iter().collect();
        //     solve_map.sort_by_key(|(f, _)| (Some(f) != primary_parent, *f));
        //     for (_, frs_vec) in &mut solve_map {
        //         frs_vec.sort_by_key(|(f, _, _)| *f);
        //         for (flags, &rank, solve) in frs_vec {
        //             let puzzle_cat = PuzzleCategory {
        //                 base: puzzle_base.clone(),
        //                 flags: **flags,
        //             };

        //             target_rows.push(Row {
        //                 solve: (*solve).clone(),
        //                 has_primary,
        //                 puzzle_base_url: puzzle_base.url_path(),
        //                 puzzle_base_name: puzzle_base.name(),
        //                 puzzle_cat_url: puzzle_cat.url_path(),
        //                 flag_modifiers: flags.emoji_string(),
        //                 rank: rank as i32,
        //                 solve_url: solve.url_path(),
        //             });
        //         }
        //     }

        //     if has_primary {
        //         speedsolves.push(target_rows);
        //     } else {
        //         speedsolves_non_primary.push(target_rows);
        //     }
        // }

        // speedsolves.extend(speedsolves_non_primary);

        // crate::render_html_template(
        //     "solver.html",
        //     &self.user,
        //     serde_json::json!({
        //         "user_id": self.target_user.id,
        //         "name": name,
        //         "can_edit": self.can_edit,
        //         "table_rows": speedsolves,
        //     }),
        // )

        todo!()
    }
}

#[derive(serde::Deserialize)]
pub struct GlobalLeaderboard {}

pub struct GlobalLeaderboardResponse {
    user: Option<User>,
}

impl RequestBody for GlobalLeaderboard {
    type Response = GlobalLeaderboardResponse;

    async fn request(
        self,
        _state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        Ok(GlobalLeaderboardResponse { user })
    }
}

impl IntoResponse for GlobalLeaderboardResponse {
    fn into_response(self) -> Response<Body> {
        crate::render_html_template("index.html", &self.user, serde_json::json!({}))
    }
}
