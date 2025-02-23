use axum::body::Body;
use axum::response::{IntoResponse, Response};

use crate::db::{
    Category, CategoryQuery, Event, MainPageQuery, ProgramQuery, Puzzle, PuzzleId, RankedFullSolve,
    User, UserId, VariantId, VariantQuery,
};
use crate::error::AppError;
use crate::traits::RequestBody;
use crate::AppState;

use super::global_leaderboard::{
    GlobalLeaderboardTable, LeaderboardEvent, LeaderboardTableColumns, LeaderboardTableResponse,
    SolveTableRow,
};

#[derive(serde::Deserialize)]
pub struct PuzzleLeaderboard {
    id: PuzzleId,
}

pub struct PuzzleLeaderboardResponse {
    user: Option<User>,

    puzzle: Puzzle,
    variants: Vec<CombinedVariant>,
}

impl RequestBody for PuzzleLeaderboard {
    type Response = PuzzleLeaderboardResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let puzzle = state.get_puzzle(self.id).await?.ok_or(AppError::NotFound)?;

        let variants = state.get_puzzle_combined_variants(puzzle.id).await?;

        Ok(PuzzleLeaderboardResponse {
            user,

            puzzle,
            variants,
        })
    }
}

impl IntoResponse for PuzzleLeaderboardResponse {
    fn into_response(self) -> Response<Body> {
        crate::render_html_template(
            "puzzle.html",
            &self.user,
            serde_json::json!({
                "puzzle": self.puzzle,
                "variants": self.variants,
            }),
        )
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct CombinedVariant {
    pub name: String,
    pub variant_abbr: Option<String>,
    pub program: Option<&'static str>,
}
impl CombinedVariant {
    pub fn new(
        variant_name: Option<String>,
        variant_abbr: Option<String>,
        variant_material_by_default: Option<bool>,
        program_material: bool,
    ) -> Self {
        let nondefault_material = variant_material_by_default.unwrap_or(false) != program_material;
        let material_or_virtual = match program_material {
            true => "Material",
            false => "Virtual",
        };
        let name = match variant_name {
            Some(variant_name) => {
                if nondefault_material {
                    format!("{material_or_virtual} {variant_name}")
                } else {
                    variant_name
                }
            }
            None => format!("{material_or_virtual}"),
        };
        let program = nondefault_material.then(|| match program_material {
            true => "material",
            false => "virtual",
        });

        Self {
            name,
            variant_abbr,
            program,
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct PuzzleLeaderboardTable {
    pub id: PuzzleId,

    pub event: Option<LeaderboardEvent>,
    pub filters: Option<bool>,
    pub macros: Option<bool>,

    pub variant: Option<VariantQuery>,
    pub program: Option<ProgramQuery>,
}

impl RequestBody for PuzzleLeaderboardTable {
    type Response = LeaderboardTableResponse;

    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let global = GlobalLeaderboardTable {
            event: self.event,
            filters: self.filters,
            macros: self.macros,
        };
        let main_page_query = global.main_page_query();
        let puzzle = state.get_puzzle(self.id).await?.ok_or(AppError::NotFound)?;

        let solves = state
            .get_event_leaderboard(
                &puzzle,
                &match main_page_query {
                    MainPageQuery::Speed {
                        average,
                        blind,
                        filters,
                        macros,
                        one_handed,
                    } => CategoryQuery::Speed {
                        average,
                        blind,
                        filters,
                        macros,
                        one_handed,
                        variant: self.variant.unwrap_or_default(),
                        program: self.program.unwrap_or_default(),
                    },
                    MainPageQuery::Fmc { computer_assisted } => {
                        CategoryQuery::Fmc { computer_assisted }
                    }
                },
            )
            .await?;

        Ok(LeaderboardTableResponse {
            table_rows: solves
                .into_iter()
                .map(|RankedFullSolve { rank, solve }| {
                    let event = Event {
                        puzzle: puzzle.clone(),
                        category: match &main_page_query {
                            MainPageQuery::Speed {
                                average,
                                blind,
                                filters,
                                macros,
                                one_handed,
                            } => {
                                let default_filters = match &solve.variant {
                                    Some(v) => v.primary_filters,
                                    None => puzzle.primary_filters,
                                };
                                let default_macros = match &solve.variant {
                                    Some(v) => v.primary_macros,
                                    None => puzzle.primary_macros,
                                };
                                Category::Speed {
                                    average: *average,
                                    blind: *blind,
                                    filters: filters.unwrap_or(default_filters),
                                    macros: macros.unwrap_or(default_macros),
                                    one_handed: *one_handed,
                                    variant: solve.variant.clone(),
                                    material: solve.program.material,
                                }
                            }
                            MainPageQuery::Fmc { computer_assisted } => Category::Fmc {
                                computer_assisted: *computer_assisted,
                            },
                        },
                    };
                    SolveTableRow::new(&event, &solve, Some(rank), None)
                })
                .collect(),

            columns: LeaderboardTableColumns {
                event: false,
                rank: true,
                solver: true,
                record_holder: false,
                speed_cs: matches!(main_page_query, MainPageQuery::Speed { .. }),
                move_count: matches!(main_page_query, MainPageQuery::Fmc { .. }),
                date: true,
                program: true,
                total_solvers: false,
            },
        })
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
