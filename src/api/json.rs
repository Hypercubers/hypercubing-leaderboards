use axum::{Json, extract::{Query, State}, http::StatusCode};
use serde::Deserialize;

use crate::{AppState, db::{CategoryQuery::{self, Speed}, CombinedVariant, Event, FullSolve, MainPageCategory, Program, ProgramQuery, Puzzle, PuzzleId, RankedFullSolve, SolveId, Variant, VariantQuery}};
use crate::db::{User, UserId};

// Query paramater for solve
#[derive(Deserialize)]
pub struct SolveQuery {
    id: SolveId
}
#[derive(Deserialize)]
pub struct PuzzleQuery {
    id: PuzzleId,
    event: Option<String>
}

#[derive(Deserialize)]
pub struct UserPBQuery {
    id: UserId,
    category: Option<CategoryQuery>
}

#[derive(Deserialize)]
pub struct EventQuery {
    event: Option<String>
}

#[derive(Deserialize)]
pub struct UserIDQuery {
    id: UserId,
}

#[derive(Deserialize)]
pub struct CategoryQueryParams {
    event: Option<String>,
    filters: Option<bool>,
    macros: Option<bool>,
    variant: Option<VariantQuery>,
    program: Option<ProgramQuery>,
}

pub fn category_query_from_params(params: CategoryQueryParams) -> CategoryQuery {
    match params.event.as_deref() {
        Some("avg") => CategoryQuery::Speed {
            average: true,
            blind: false,
            filters: params.filters,
            macros: params.macros,
            one_handed: false,
            variant: params.variant.unwrap_or(VariantQuery::Default),
            program: params.program.unwrap_or(ProgramQuery::Default),
        },

        Some("bld") => CategoryQuery::Speed {
            average: false,
            blind: true,
            filters: params.filters,
            macros: params.macros,
            one_handed: false,
            variant: params.variant.unwrap_or(VariantQuery::Default),
            program: params.program.unwrap_or(ProgramQuery::Default),
        },

        Some("oh") => CategoryQuery::Speed {
            average: false,
            blind: false,
            filters: params.filters,
            macros: params.macros,
            one_handed: true,
            variant: params.variant.unwrap_or(VariantQuery::Default),
            program: params.program.unwrap_or(ProgramQuery::Default),
        },

        Some("fmcca") => CategoryQuery::Fmc {
            computer_assisted: true,
        },

        Some("fmc") | None => CategoryQuery::Fmc {
            computer_assisted: false,
        },

        _ => CategoryQuery::default(),
    }
}

pub fn event_to_category_query(event: Option<String>) -> CategoryQuery {
    match event {
        Some(text) => {
            Speed {
                average: match text.as_str() {
                    "avg" => true,
                    _ => false
                },
                blind: match text.as_str() {
                    "bld" => true,
                    _ => false
                },
                filters: None,
                macros: None,
                one_handed: match text.as_str() {
                    "oh" => true,
                    _ => false
                },
                variant: VariantQuery::Default,
                program: ProgramQuery::Default,
            }
        },
        None => {CategoryQuery::default()}
    }
}

pub async fn get_json_variants(State(state): State<AppState>) -> Json<Vec<Variant>> {
    let variants = state.get_all_variants().await;
    match variants {
        Ok(var) => Json(var),
        Err(_) => Json(vec![])
    }
}

pub async fn get_json_puzzles(State(state): State<AppState>) -> Json<Vec<Puzzle>> {
    let puzzles = state.get_all_puzzles().await;
    match puzzles {
        Ok(puz) => Json(puz),
        Err(_) => Json(vec![])
    }
}

pub async fn get_json_programs(State(state): State<AppState>) -> Json<Vec<Program>> {
    let programs = state.get_all_programs().await;
    match programs {
        Ok(progs) => Json(progs),
        Err(_) => Json(vec![])
    }
}

pub async fn get_puzzle_variants(State(state): State<AppState>, Query(params): Query<PuzzleQuery>) -> Json<Vec<CombinedVariant>> {
    let variants = state.get_puzzle_combined_variants(params.id).await;
    match variants {
        Ok(v) => Json(v),
        Err(_) => Json(vec![])
    }
}

// returns an array of [Event, FullSolve]
pub async fn get_json_all_puzzles_leaderboard(State(state): State<AppState>, Query(params): Query<CategoryQueryParams>) -> Json<Vec<(Event, FullSolve)>> {
    let query = category_query_from_params(params);
    let records = state.get_all_puzzles_leaderboard(&query).await;
    match records {
        Ok(rec) => Json(rec),
        Err(_) => Json(vec![])
    }
}

// returns a FullSolve given an ID
pub async fn get_json_solve(State(state): State<AppState>, Query(params): Query<SolveQuery>) -> Result<Json<FullSolve>, StatusCode> {
    let id = params.id;
    let solve = state.get_solve(id).await;
    match solve {
        Ok(s) => Ok(Json(s)),
        Err(_) => Err(StatusCode::NOT_FOUND)
    }
}

#[axum::debug_handler]
// returns a list of RankedFullSolve given a puzzle and category in a PuzzleQuery
pub async fn get_json_puzzle(State(state): State<AppState>, Query(params): Query<PuzzleQuery>) -> Result<Json<Vec<RankedFullSolve>>, StatusCode> {
    let id = state.get_puzzle(params.id).await;
    // let cat: CategoryQuery::Default;
    match id {
        Ok(Some(puzzle)) => {
            let cat = event_to_category_query(params.event);
            let rankings = state.get_event_leaderboard(&puzzle, &cat).await;
            match rankings {
                Ok(r) => Ok(Json(r)),
                Err(_) => Err(StatusCode::NOT_FOUND)
            }
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_json_user_pbs(State(state): State<AppState>, Query(params): Query<UserPBQuery>) -> Result<Json<Vec<(MainPageCategory, RankedFullSolve)>>, StatusCode> {
    let category = params.category;
    let cat = match category {
        Some(cat) => cat,
        None => {CategoryQuery::default()}
    };
    let pbs = state.get_solver_pbs(params.id, &cat).await;
    match pbs {
        Ok(p) => Ok(Json(p)),
        Err(_) => Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_json_user_submissions(State(state): State<AppState>, Query(params): Query<UserIDQuery>) -> Result<Json<Vec<FullSolve>>, StatusCode> {
    let id = params.id;
    let submissions = state.get_solver_submissions(id).await;
    match submissions {
        Ok(s) => Ok(Json(s)),
        Err(_) => Err(StatusCode::NOT_FOUND)
    }
}
