use axum::{Json, extract::State};
use serde::Serialize;

use crate::{AppState, db::{self, CategoryQuery::{self, Speed}, Event, FullSolve, ProgramQuery, Puzzle, VariantQuery}};



#[derive(Serialize)]
pub struct Message {
    text: String,
}


pub async fn get_json_hello() -> Json<Message> {
    Json(Message {
        text: String::from("Hello World"),
    })
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
