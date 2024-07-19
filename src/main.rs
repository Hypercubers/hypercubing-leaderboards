use crate::traits::RequestBody;
use axum::{
    routing::{get, post},
    Router,
};
use parking_lot::Mutex;
use poise::serenity_prelude::*;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::collections::HashMap;
use std::sync::Arc;

mod api;
mod db;
mod error;
mod html;
mod traits;
mod util;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    // ephemeral database mapping user database id to otp
    otps: Arc<Mutex<HashMap<i32, db::auth::Otp>>>,
    discord_http: Arc<Http>,
    discord_cache: Arc<Cache>,
    discord_shard: ShardMessenger,
}

impl CacheHttp for AppState {
    fn http(&self) -> &Http {
        &self.discord_http
    }

    // Provided method
    fn cache(&self) -> Option<&Arc<Cache>> {
        Some(&self.discord_cache)
    }
}

impl AsRef<ShardMessenger> for AppState {
    fn as_ref(&self) -> &ShardMessenger {
        &self.discord_shard
    }
}

#[allow(dead_code)]
fn assert_send(_: impl Send) {}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = std::env!("DISCORD_TOKEN"); //.expect("Expected a token in the environment");
                                            // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::non_privileged();

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    //if let Err(why) = client.start().await {
    //    println!("Client error: {why:?}");
    // }

    let shard_manager = client.shard_manager.clone(); // it's an Arc<>
    let discord_http = client.http.clone();
    let discord_cache = client.cache.clone();

    tokio::spawn(async move { client.start().await });

    let discord_shard = loop {
        if let Some(runner) = shard_manager.runners.lock().await.iter().next() {
            break runner.1.runner_tx.clone();
        }
    };

    /*assert_send(html::boards::PuzzleLeaderboard::as_handler_query(
        todo!(),
        todo!(),
        todo!(),
    ));*/

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
        discord_http,
        discord_cache,
        discord_shard,
    };

    let app = Router::new()
        /*.route(
            "/api/v1/auth/request-otp",
            post(api::auth::user_request_otp),
        )
        .route(
            "/api/v1/auth/request-token",
            post(api::auth::user_request_token),
        )
        .route(
            "/api/v1/upload-solve",
            post(api::upload::UploadSolve::as_handler_file),
        )
        .route(
            "/api/v1/upload-solve-external",
            post(api::upload::UploadSolveExternal::as_handler_file),
            //post(api::upload::UploadSolveExternal::show_all),
        )*/
        .route(
            "/puzzle",
            get(html::boards::PuzzleLeaderboard::as_handler_query),
        )
        .route(
            "/solver",
            get(html::boards::SolverLeaderboard::as_handler_query),
        )
        .route(
            "/upload-external",
            get(html::forms::upload_external)
                .post(api::upload::UploadSolveExternal::as_multipart_form_handler),
        )
        .route(
            "/sign-in",
            get(html::forms::sign_in), //.post(html::auth::SignInForm::as_multipart_form_handler),
        )
        .route(
            "/sign-in-discord",
            post(html::auth_discord::SignInDiscordForm::as_multipart_form_handler),
        )
        .route("/js/form.js", get(include_str!(".././js/form.js")))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Engaged");
    axum::serve(listener, app).await.unwrap();
}
