use axum::{Json, extract::{Query, State}, http::StatusCode};
use serde::Deserialize;

use crate::{AppState, db::{CategoryQuery::{self, Speed}, Event, FullSolve, ProgramQuery, Puzzle, PuzzleId, RankedFullSolve, SolveId, VariantQuery}};

// Query paramater for solve
#[derive(Deserialize)]
pub struct SolveQuery {
    id: SolveId
}
#[derive(Deserialize)]
pub struct PuzzleQuery {
    id: PuzzleId,
    category: Option<CategoryQuery>
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
            let category = params.category;

            let cat = match category {
                Some(cat) => cat,
                None => {CategoryQuery::default()}

            };



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
