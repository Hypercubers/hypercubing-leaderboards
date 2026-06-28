use axum::{Json, extract::{Query, State}, http::StatusCode};
use serde::Deserialize;

use crate::{AppState, db::{self, CategoryQuery::{self, Speed}, Event, FullSolve, ProgramQuery, Puzzle, SolveId, VariantQuery}};

// Query paramater for solve
#[derive(Deserialize)]
pub struct SolveQuery {
    id: SolveId
}




pub async fn get_json_puzzles(State(state): State<AppState>) -> Json<Vec<Puzzle>> {
    let puzzles = state.get_all_puzzles().await;
    match puzzles {
        Ok(puz) => Json(puz),
        Err(_) => Json(vec![])
    }
}

pub async fn get_json_all_puzzles_leaderboard(State(state): State<AppState>) -> Json<Vec<(Event, FullSolve)>> {
    let query = Speed {
            average: false,
            blind: false,
            filters: None,
            macros: None,
            one_handed: false,
            variant: VariantQuery::Default,
            program: ProgramQuery::Default,
        };
    let records = state.get_all_puzzles_leaderboard(&query).await;
    match records {
        Ok(rec) => Json(rec),
        Err(_) => Json(vec![])
    }
}

pub async fn get_json_solve(State(state): State<AppState>, Query(params): Query<SolveQuery>) -> Result<Json<FullSolve>, StatusCode> {
    let id = params.id;
    let solve = state.get_solve(id).await;
    match solve {
        Ok(s) => Ok(Json(s)),
        Err(_) => Err(StatusCode::NOT_FOUND)
    }
}
