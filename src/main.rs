use axum::{
    routing::{get, post},
    Router,
};
use parking_lot::Mutex;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::collections::HashMap;
use std::sync::Arc;

mod api;
mod db;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    // ephemeral database mapping user database id to otp
    otps: Arc<Mutex<HashMap<i32, api::auth::Otp>>>,
}

#[tokio::main]
async fn main() {
    let db_connection_str = std::env!("DATABASE_URL");

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let state = AppState {
        pool,
        otps: Default::default(),
    };

    let app = Router::new()
        .route(
            "/api/v1/auth/request-otp",
            post(crate::api::auth::user_request_otp),
        )
        .route(
            "/api/v1/auth/request-token",
            post(crate::api::auth::user_request_token),
        )
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Engaged");
    axum::serve(listener, app).await.unwrap();
}
