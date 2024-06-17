use crate::AppState;
use axum::extract::State;
use axum::http::{header::SET_COOKIE, StatusCode};
use axum::response::{AppendHeaders, IntoResponse};
use axum::Json;
use chrono::{DateTime, TimeDelta, Utc};


#[derive(serde::Deserialize)]
pub struct UploadSolve {
    email: String,
    display_name: Option<String>,
}

pub async fn upload_solve()