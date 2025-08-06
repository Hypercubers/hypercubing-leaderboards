use std::collections::HashMap;
use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use clap::Parser;
use parking_lot::Mutex;
use poise::serenity_prelude as sy;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use sqlx::ConnectOptions;

use crate::db::UserId;
use crate::traits::RequestBody;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;
mod api;
mod cli;
mod cookies;
mod db;
mod env;
mod error;
mod html;
mod static_files;
mod traits;
mod util;

use static_files::{render_html_template, Assets, CssFiles, JsFiles, HBS};

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

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    let log_file = tracing_appender::rolling::daily("./logs", "warnings");

    tracing_subscriber::fmt()
        .with_writer(log_file)
        .with_ansi(false)
        .with_env_filter(tracing_subscriber::EnvFilter::new(&*env::RUST_LOG))
        .init();

    // Load handlebars templates.
    lazy_static::initialize(&HBS);

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = sy::GatewayIntents::non_privileged() | sy::GatewayIntents::GUILD_MEMBERS;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = sy::Client::builder(&*env::DISCORD_TOKEN, intents)
        .await
        .expect("error creating Discord client");

    let shard_manager = client.shard_manager.clone(); // it's an Arc<>
    let http = client.http.clone();
    let cache = client.cache.clone();

    tokio::spawn(async move { client.start().await });

    let shard = loop {
        if let Some(runner) = shard_manager.runners.lock().await.iter().next() {
            break runner.1.runner_tx.clone();
        }
    };

    let db_connect_options: PgConnectOptions = env::DATABASE_URL
        .parse::<PgConnectOptions>()
        .expect("invalid database connection options")
        .log_statements(tracing::log::LevelFilter::Trace);

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect_with(db_connect_options)
        .await
        .expect("error connecting to database");

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

    let mut client_slash = sy::Client::builder(&*env::DISCORD_TOKEN, intents)
        .framework(framework)
        .await
        .expect("error creating Discord client for slash commands");

    tokio::spawn(async move {
        if let Err(why) = client_slash.start().await {
            tracing::error!(?why, "Client error");
        }
    });

    match args.command {
        None | Some(cli::Command::Run) => (), // continue
        Some(cli::Command::Reset) => {
            state.reset().await.expect("error resetting database");
            std::process::exit(0);
        }
        Some(cli::Command::Migrate) => {
            state.migrate().await.expect("error migrating database");
            std::process::exit(0);
        }
        Some(cli::Command::Init) => {
            state
                .init_puzzles()
                .await
                .expect("error loading initial puzzles");
            state
                .init_solves()
                .await
                .expect("error loading initial solves");
            std::process::exit(0);
        }
        Some(cli::Command::Promote { user_id }) => {
            let user = state
                .get_user(UserId(user_id))
                .await
                .expect("error finding user")
                .expect("user not found");
            let display_name = user.to_public().display_name();
            if user.moderator {
                println!("{display_name} is already a moderator");
            } else {
                state
                    .set_moderator(UserId(user_id), true)
                    .await
                    .expect("error demoting user");
                println!("{display_name} is now a moderator");
            }
            std::process::exit(0);
        }
        Some(cli::Command::Demote { user_id }) => {
            let user = state
                .get_user(UserId(user_id))
                .await
                .expect("error finding user")
                .expect("user not found");
            let display_name = user.to_public().display_name();
            if !user.moderator {
                println!("{display_name} is already not a moderator");
            } else {
                state
                    .set_moderator(UserId(user_id), false)
                    .await
                    .expect("error demoting user");
                println!("{display_name} is no longer a moderator");
            }
            std::process::exit(0);
        }
    }

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
        .route(
            "/",
            get(html::puzzle_leaderboard::GlobalLeaderboard::as_handler_query),
        )
        .route(
            "/puzzle",
            get(html::puzzle_leaderboard::PuzzleLeaderboard::as_handler_query),
        )
        .route(
            "/solve-table/all",
            get(html::global_leaderboard::GlobalLeaderboardTable::as_handler_query),
        )
        .route(
            "/solve-table/puzzle",
            get(html::puzzle_leaderboard::PuzzleLeaderboardTable::as_handler_query),
        )
        .route(
            "/solve-table/user",
            get(html::user_page::SolverLeaderboardTable::as_handler_query),
        )
        .route(
            "/solve-table/my-submissions",
            get(html::my_submissions::MySubmissionsTable::as_handler_query),
        )
        .route(
            "/solver",
            get(html::puzzle_leaderboard::SolverLeaderboard::as_handler_query),
        )
        .route("/solve", get(html::solve::SolvePage::as_handler_query))
        .route(
            "/submit",
            get(html::forms::SubmitSolve::as_handler_query)
                .post(api::upload::ManualSubmitSolve::as_multipart_form_handler),
        )
        .route("/sign-in", get(html::sign_in::SignInPage::as_handler_query))
        .route(
            "/sign-out",
            get(html::sign_out::SignOutPage::as_handler_query),
        )
        .route(
            "/sign-in-discord",
            post(html::auth_discord::SignInDiscordForm::as_multipart_form_handler),
        )
        .route(
            "/my-submissions",
            get(html::my_submissions::MySubmissionsPage::as_handler_query),
        )
        .route(
            "/settings",
            get(html::forms::Settings::as_handler_query)
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
        // .route(
        //     "/update-solve-program",
        //     post(api::upload::UpdateSolveProgramVersionId::as_multipart_form_handler),
        // )
        .nest_service("/js", axum_embed::ServeEmbed::<JsFiles>::new())
        .nest_service("/css", axum_embed::ServeEmbed::<CssFiles>::new())
        .nest_service("/assets", axum_embed::ServeEmbed::<Assets>::new())
        .layer(tower_governor::GovernorLayer {
            config: Arc::new(
                tower_governor::governor::GovernorConfigBuilder::default()
                    .burst_size(10)
                    .finish()
                    .expect("error finishing tower_governor config"),
            ),
        })
        .fallback(html::not_found::handler_query)
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("error binding port 3000");
    tracing::info!("Engaged");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .expect("error serving web service");
}
