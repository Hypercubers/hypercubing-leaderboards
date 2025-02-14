use std::collections::HashMap;
use std::sync::Arc;

use axum::http::header::{HeaderMap, CONTENT_TYPE};
use axum::http::HeaderValue;
use axum::response::Html;
use axum::routing::{get, post};
use axum::Router;
use parking_lot::Mutex;
use poise::serenity_prelude as sy;
use sqlx::postgres::{PgPool, PgPoolOptions};

use crate::db::user::UserId;
use crate::traits::RequestBody;

#[macro_use]
extern crate lazy_static;

mod api;
mod db;
mod error;
mod html;
mod templates;
mod traits;
mod util;

use templates::HBS;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    // ephemeral database mapping user database id to otp
    otps: Arc<Mutex<HashMap<UserId, db::auth::Otp>>>,
    discord: Option<DiscordAppState>,
}

#[derive(Clone)]
struct DiscordAppState {
    http: Arc<sy::Http>,
    cache: Arc<sy::Cache>,
    shard: sy::ShardMessenger,
}

impl sy::CacheHttp for DiscordAppState {
    fn http(&self) -> &sy::Http {
        &self.http
    }

    // Provided method
    fn cache(&self) -> Option<&Arc<sy::Cache>> {
        Some(&self.cache)
    }
}

impl AsRef<sy::Http> for DiscordAppState {
    fn as_ref(&self) -> &sy::Http {
        &self.http
    }
}

impl AsRef<sy::ShardMessenger> for DiscordAppState {
    fn as_ref(&self) -> &sy::ShardMessenger {
        &self.shard
    }
}

#[allow(dead_code)]
fn assert_send(_: impl Send) {}

/// Returns a [`HeaderMap`] with a MIME type.
fn mime(m: &'static str) -> HeaderMap {
    HeaderMap::from_iter([(CONTENT_TYPE, HeaderValue::from_static(&m))])
}

/// Fallback route handler that returns a 404 error.
async fn fallback(_uri: axum::http::Uri) -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::NOT_FOUND, "404".to_string())
}

#[tokio::main]
async fn main() {
    let log_file = tracing_appender::rolling::daily("./logs", "warnings");

    tracing_subscriber::fmt()
        .with_writer(log_file)
        .with_ansi(false)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            dotenvy::var("RUST_LOG").expect("has it"),
        ))
        .init();

    // Load handlebars templates.
    lazy_static::initialize(&HBS);

    // Configure the client with your Discord bot token in the environment.
    let token = dotenvy::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = sy::GatewayIntents::non_privileged() | sy::GatewayIntents::GUILD_MEMBERS;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = sy::Client::builder(&token, intents)
        .await
        .expect("Err creating client");

    let shard_manager = client.shard_manager.clone(); // it's an Arc<>
    let http = client.http.clone();
    let cache = client.cache.clone();

    tokio::spawn(async move { client.start().await });

    let shard = loop {
        if let Some(runner) = shard_manager.runners.lock().await.iter().next() {
            break runner.1.runner_tx.clone();
        }
    };

    /*assert_send(html::boards::PuzzleLeaderboard::as_handler_query(
        todo!(),
        todo!(),
        todo!(),
    ));*/

    let db_connection_str = dotenvy::var("DATABASE_URL").expect("should have database url");

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
        discord: Some(DiscordAppState { http, cache, shard }),
    };

    let framework = {
        let state = state.clone();
        poise::Framework::builder()
            .options(poise::FrameworkOptions {
                commands: vec![
                    api::profile::update_profile(),
                    api::moderation::verify_speed(),
                ],
                ..Default::default()
            })
            .setup(|ctx, _ready, framework| {
                Box::pin(async move {
                    //poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    for guild_id in ctx.cache.guilds() {
                        tracing::info!(?guild_id, "registering in guild");
                        poise::builtins::register_in_guild(
                            ctx,
                            &framework.options().commands,
                            guild_id,
                        )
                        .await?;
                    }
                    Ok(state)
                })
            })
            .build()
    };

    let mut client_slash = sy::Client::builder(&token, intents)
        .framework(framework)
        .await
        .expect("Err creating client");

    tokio::spawn(async move {
        if let Err(why) = client_slash.start().await {
            tracing::error!(?why, "Client error");
        }
    });

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
            //post(api::upload::UploadSolveExternal::show_all), // api endpoint for sign out
        )*/
        .route("/", get(html::boards::GlobalLeaderboard::as_handler_query))
        .route(
            "/puzzle",
            get(html::boards::PuzzleLeaderboard::as_handler_query),
        )
        .route(
            "/solver",
            get(html::boards::SolverLeaderboard::as_handler_query),
        )
        .route("/solve", get(html::solve::SolvePage::as_handler_query))
        .route(
            "/upload-external",
            get(html::forms::UploadSolveExternal::as_handler_query)
                .post(api::upload::UploadSolveExternal::as_multipart_form_handler),
        )
        .route(
            "/sign-in",
            get(Html(include_str!("../html/sign-in.html"))), //.post(html::auth::SignInForm::as_multipart_form_handler),
        )
        .route("/signed-out", get(html::auth::sign_out))
        .route(
            "/sign-in-discord",
            post(html::auth_discord::SignInDiscordForm::as_multipart_form_handler),
        )
        .route(
            "/update-profile",
            get(html::forms::UpdateProfile::as_handler_query)
                .post(api::profile::UpdateProfile::as_multipart_form_handler),
        )
        .route(
            "/update-solve-video-url",
            post(api::upload::UpdateSolveVideoUrl::as_multipart_form_handler),
        )
        .route(
            "/update-solve-speed-cs",
            post(api::upload::UpdateSolveSpeedCs::as_multipart_form_handler),
        )
        .route(
            "/update-solve-category",
            post(api::upload::UpdateSolveCategory::as_multipart_form_handler),
        )
        .route(
            "/update-solve-move-count",
            post(api::upload::UpdateSolveMoveCount::as_multipart_form_handler),
        )
        .route(
            "/update-solve-program",
            post(api::upload::UpdateSolveProgramVersionId::as_multipart_form_handler),
        )
        .route(
            "/js/form.js",
            get((mime("text/javascript"), include_str!("../js/form.js"))),
        )
        .route(
            "/js/solve_table.js",
            get((
                mime("text/javascript"),
                include_str!("../js/solve_table.js"),
            )),
        )
        .route(
            "/js/redirect-here.js",
            get((
                mime("text/javascript"),
                include_str!("../js/redirect-here.js"),
            )),
        )
        .route(
            "/css/solve_table.css",
            get((mime("text/css"), include_str!("../css/solve_table.css"))),
        )
        .route(
            "/css/edit_form.css",
            get((mime("text/css"), include_str!("../css/edit_form.css"))),
        )
        .layer(tower_governor::GovernorLayer {
            config: Arc::new(tower_governor::governor::GovernorConfig::default()),
        })
        .with_state(state)
        .fallback(fallback);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Engaged");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
}
