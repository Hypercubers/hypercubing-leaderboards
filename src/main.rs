#[allow(unused_extern_crates)]
extern crate axum_typed_multipart_macros; // version must be pinned
#[allow(unused_extern_crates)]
extern crate tracing_appender; // used in debug mode but not release

use std::collections::HashMap;
use std::os::unix::process::CommandExt;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use cf_turnstile::TurnstileClient;
use clap::Parser;
use poise::serenity_prelude as sy;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use sqlx::ConnectOptions;
use tokio::sync::{mpsc, Mutex};

use crate::api::auth::Otp;
use crate::error::{AppError, AppResult};
use crate::traits::{PoiseCtx, PoiseCtxExt, RequestBody};

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;
mod api;
mod cli;
mod cookies;
mod db;
mod discord;
mod email;
mod env;
mod error;
mod html;
mod routes;
mod static_files;
mod traits;
mod util;

use static_files::{render_html_template, render_template, HBS};

#[derive(Clone)]
struct AppState {
    /// Database connection pool.
    pool: PgPool,
    /// Ephemeral database of OTPs, indexed by device code.
    otps: Arc<Mutex<HashMap<String, Otp>>>,
    /// Discord bot state.
    discord: Option<DiscordAppState>,
    /// Cloudflare Turnstile state.
    turnstile: Option<Arc<TurnstileClient>>,

    /// Whether to block logins.
    block_logins: Arc<AtomicBool>,
    /// Whether to block solve submissions.
    block_solve_submissions: Arc<AtomicBool>,
    /// Whether to block all non-read user actions.
    block_user_actions: Arc<AtomicBool>,
    /// Whether to block all non-read moderator actions.
    block_moderator_actions: Arc<AtomicBool>,

    /// Shutdown signal.
    shutdown_tx: mpsc::Sender<String>,
    /// Whether a restart was requested after shutdown.
    restart_requested: Arc<AtomicBool>,
}

impl AppState {
    /// Returns the Discord bot state, or an error if the Discord bot is not
    /// logged in.
    pub fn try_discord(&self) -> AppResult<&DiscordAppState> {
        self.discord.as_ref().ok_or(AppError::NoDiscord)
    }

    /// Requests a shutdown.
    pub async fn request_shutdown(&self, reason: String) {
        if let Err(e) = self.shutdown_tx.send(reason).await {
            tracing::error!("Shutdown channel error: {e}");
        }
    }
    /// Requests a shutdown & restart.
    pub async fn request_restart(&self, reason: String) {
        self.restart_requested
            .store(true, std::sync::atomic::Ordering::Relaxed);
        self.request_shutdown(reason).await;
    }

    fn error_if_blocked(b: &AtomicBool) -> AppResult<()> {
        match b.load(std::sync::atomic::Ordering::Relaxed) {
            true => Err(AppError::TemporarilyBlocked),
            false => Ok(()),
        }
    }

    /// Returns an error if logins are currently blocked.
    pub fn check_allow_logins(&self) -> AppResult<()> {
        self.check_allow_user_actions()?;
        Self::error_if_blocked(&self.block_logins)?;
        Ok(())
    }

    /// Returns an error if submissions are currently blocked.
    pub fn check_allow_submissions(&self) -> AppResult<()> {
        self.check_allow_user_actions()?;
        Self::error_if_blocked(&self.block_solve_submissions)?;
        Ok(())
    }

    /// Returns an error if non-read user actions are currently blocked.
    pub fn check_allow_user_actions(&self) -> AppResult<()> {
        Self::error_if_blocked(&self.block_user_actions)
    }

    /// Returns an error if non-read admin operations are currently blocked.
    pub fn check_allow_moderator_actions(&self) -> AppResult<()> {
        Self::error_if_blocked(&self.block_moderator_actions)
    }

    /// Returns an error if the operation is currently blocked.
    pub fn check_allow_edit(&self, editor: &crate::db::User) -> AppResult<()> {
        Self::error_if_blocked(&self.block_moderator_actions)?;
        if !editor.moderator {
            Self::error_if_blocked(&self.block_user_actions)?;
        }
        Ok(())
    }
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
    let args = cli::CliArgs::parse();

    // Initialize logging
    {
        #[cfg(not(debug_assertions))]
        let (writer, ansi) = (tracing_appender::rolling::daily("./logs", "log"), false);
        #[cfg(debug_assertions)]
        let (writer, ansi) = (std::io::stdout, true);

        tracing_subscriber::fmt()
            .with_writer(writer)
            .with_ansi(ansi)
            .with_env_filter(tracing_subscriber::EnvFilter::new(&*env::RUST_LOG))
            .init();
    }

    tracing::info!(
        "Starting {} {}",
        env!("CARGO_PKG_NAME"),
        env!("VERGEN_GIT_SHA"),
    );

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

    let (shutdown_tx, shutdown_rx) = mpsc::channel(1);

    let state = AppState {
        pool,
        otps: Default::default(),
        discord: Some(DiscordAppState { http, cache, shard }),
        turnstile: Some(Arc::new(TurnstileClient::new(
            env::TURNSTILE_SECRET_KEY.clone().into(),
        ))),

        block_logins: Arc::new(AtomicBool::new(false)),
        block_solve_submissions: Arc::new(AtomicBool::new(false)),
        block_user_actions: Arc::new(AtomicBool::new(false)),
        block_moderator_actions: Arc::new(AtomicBool::new(false)),

        shutdown_tx,
        restart_requested: Arc::new(AtomicBool::new(false)),
    };

    let framework = {
        let state = state.clone();
        poise::Framework::builder()
            .options(poise::FrameworkOptions {
                commands: vec![
                    // User commands
                    discord::user::user(),
                    // Verify commands
                    discord::verify::accept(),
                    discord::verify::reject(),
                    discord::verify::unverify(),
                    // Admin commands
                    discord::admin::version(),
                    discord::admin::shutdown(),
                    discord::admin::restart(),
                    discord::admin::update(),
                    // Block/unblock commands
                    discord::panic::panic(),
                ],
                event_handler: |_sy_ctx, ev, _ctx, _| {
                    Box::pin(async move {
                        match ev {
                            sy::FullEvent::Ready { .. } => tracing::info!("Discord bot is ready"),
                            _ => (),
                        }
                        Ok(())
                    })
                },
                ..Default::default()
            })
            .setup(|ctx, _ready, framework| {
                Box::pin(async move {
                    //poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    for guild_id in ctx.cache.guilds() {
                        poise::builtins::register_in_guild(
                            ctx,
                            &framework.options().commands,
                            guild_id,
                        )
                        .await?;
                        let guild_name = ctx.http.get_guild(guild_id).await?.name;
                        tracing::info!("Discord bot is registered in guild {guild_name}");
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
            tracing::error!(?why, "Discord client error");
        }
    });

    args.command
        .unwrap_or_default()
        .execute(state, shutdown_rx)
        .await
        .expect("error executing command");
}

async fn run_web_server(state: AppState, mut shutdown_rx: mpsc::Receiver<String>) {
    let restart_requested = Arc::clone(&state.restart_requested);

    let app = routes::router()
        .layer(axum_helmet::HelmetLayer::new(
            axum_helmet::Helmet::new()
                .add(
                    axum_helmet::ContentSecurityPolicy::new()
                        .default_src(vec!["'self'"])
                        .frame_src(vec![
                            "'self'",
                            "https://challenges.cloudflare.com/", // cloudflare turnstile
                            // video URLs
                            "https://youtube.com/",
                            "https://www.youtube.com/",
                            "https://youtube-nocookie.com/",
                            "https://www.youtube-nocookie.com/",
                            "https://youtu.be/",
                            "https://s.ytimg.com/",
                        ])
                        .base_uri(vec!["'self'"])
                        .font_src(vec!["'self'", "https:", "data:"])
                        .form_action(vec!["'self'"])
                        .img_src(vec![
                            "'self'",
                            "https:",
                            "data:",
                            "https://s.ytimg.com/", // YouTube video thumbnails
                        ])
                        .object_src(vec!["'none'"])
                        .script_src(vec![
                            "'self'",
                            "https://cdn.jsdelivr.net/", // jquery
                            "https://challenges.cloudflare.com/", // cloudflare turnstile
                            // iconify
                            "https://code.iconify.design/",
                            "https://api.iconify.design/",
                        ])
                        .style_src(vec!["'self'", "https:", "'unsafe-inline'"])
                        .upgrade_insecure_requests(),
                )
                .add(axum_helmet::CrossOriginOpenerPolicy::same_origin())
                .add(axum_helmet::CrossOriginResourcePolicy::same_origin())
                .add(axum_helmet::ReferrerPolicy::strict_origin_when_cross_origin())
                .add(axum_helmet::StrictTransportSecurity::default())
                .add(axum_helmet::XContentTypeOptions::nosniff())
                .add(axum_helmet::XFrameOptions::same_origin()),
        ))
        .layer(tower_governor::GovernorLayer {
            config: Arc::new(
                tower_governor::governor::GovernorConfigBuilder::default()
                    .burst_size(30)
                    .finish()
                    .expect("error finishing tower_governor config"),
            ),
        })
        .fallback(html::not_found::handler_query)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("error binding port 3000");

    tracing::info!("Web server is ready");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(async move {
        match shutdown_rx.recv().await {
            Some(reason) => tracing::info!("Gracefully shutting down web server: {reason}"),
            None => tracing::error!("Web server shutdown channel disconnected"),
        }
    })
    .await
    .expect("error serving web service");

    if restart_requested.load(std::sync::atomic::Ordering::Relaxed) {
        // Don't use `std::env::current_exe()` because it will update when the
        // executable is moved.
        let this_executable = std::env::args().next().expect("unknown executable name");
        tracing::info!("Restarting web server by running {this_executable:?}");
        let error = std::process::Command::new(this_executable).exec(); // should not return
        tracing::error!("Could not restart: {error}");
    }
}
