use axum::{Json, extract::State};
use serde::Serialize;

use crate::{AppState, db::{self, Puzzle}};



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
