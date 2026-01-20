use tokio::sync::mpsc;

use crate::AppState;
use crate::db::UserId;

/// Hyperspeedcube leaderboards server.
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct CliArgs {
    #[command(subcommand)]
    pub command: Option<CliCommand>,
}

#[derive(clap::Subcommand, Debug, Default)]
pub(crate) enum CliCommand {
    /// Runs the web server
    #[default]
    Run,
    /// Resets the database
    Reset,
    /// Migrates the database to the latest schema
    Migrate,
    /// Initializes the database from solves.csv (default)
    Init,
    /// Manage users
    User {
        #[command(subcommand)]
        user_command: CliUserCommand,
    },
    /// Manage puzzles
    Puzzle {
        #[command(subcommand)]
        puzzle_command: CliPuzzleCommand,
    },
}

impl CliCommand {
    pub async fn execute(
        self,
        state: AppState,
        shutdown_rx: mpsc::Receiver<String>,
    ) -> eyre::Result<()> {
        match self {
            CliCommand::Run => {
                crate::run_web_server(state, shutdown_rx).await;
                println!("Web server has terminated.");
                Ok(())
            }
            CliCommand::Reset => {
                println!(
                    "Enter 'reset' (lowercase, without quotes) \
                     below to reset the database completely."
                );
                println!("Enter anything else to exit the program.");
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf)?;
                if buf.trim() == "reset" {
                    println!("Resetting database ...");
                    state.reset().await?;
                    println!("Database has been reset.");
                } else {
                    println!("Canceled. Database was not reset.");
                }
                Ok(())
            }
            CliCommand::Migrate => {
                state.migrate().await?;
                println!("Database has been migrated.");
                Ok(())
            }
            CliCommand::Init => {
                state.init_from_csv().await?;
                println!("Database has been initialized.");
                Ok(())
            }
            CliCommand::User { user_command } => user_command.execute(&state).await,
            CliCommand::Puzzle { puzzle_command } => puzzle_command.execute(&state).await,
        }
    }
}

#[derive(clap::Subcommand, Debug, Default)]
pub(crate) enum CliUserCommand {
    /// Lists all users
    #[default]
    List,
    /// Shows info for a particular user
    Info { user_id: i32 },
    /// Sets or clears the Discord ID of a user
    SetDiscord {
        user_id: i32,
        discord_id: Option<u64>,
    },
    /// Sets or clears the email address of a user
    SetEmail { user_id: i32, email: Option<String> },
    /// Promotes a user to a moderator
    Promote { user_id: i32 },
    /// Demotes a moderator
    Demote { user_id: i32 },
}

impl CliUserCommand {
    pub async fn execute(self, state: &AppState) -> eyre::Result<()> {
        let cli = state.get_cli_dummy_user().await?;

        match self {
            CliUserCommand::List => {
                serde_json::to_writer_pretty(std::io::stdout(), &state.get_all_users().await?)?;
            }
            CliUserCommand::Info { user_id } => serde_json::to_writer_pretty(
                std::io::stdout(),
                &state.get_user(UserId(user_id)).await?,
            )?,
            CliUserCommand::SetDiscord {
                user_id,
                discord_id,
            } => {
                let target = UserId(user_id);
                state
                    .update_user_discord_id(&cli, target, discord_id)
                    .await?;
                println!("Set user Discord ID for user #{user_id} to {discord_id:?}");
            }
            CliUserCommand::SetEmail { user_id, email } => {
                let target = UserId(user_id);
                state.update_user_email(&cli, target, email.clone()).await?;
                println!("Set user email for user #{user_id} to {email:?}");
            }
            CliUserCommand::Promote { user_id } => {
                let user = state.get_user(UserId(user_id)).await?;
                let name = user.to_public().display_name();
                if user.moderator {
                    println!("{name} is already a moderator");
                } else {
                    let target = UserId(user_id);
                    state.update_user_is_moderator(&cli, target, true).await?;
                    println!("{name} is now a moderator");
                }
            }
            CliUserCommand::Demote { user_id } => {
                let user = state.get_user(UserId(user_id)).await?;
                let name = user.to_public().display_name();
                if !user.moderator {
                    println!("{name} is already not a moderator");
                } else {
                    let target = UserId(user_id);
                    state.update_user_is_moderator(&cli, target, false).await?;
                    println!("{name} is no longer a moderator");
                }
            }
        }
        Ok(())
    }
}

#[derive(clap::Subcommand, Debug, Default)]
pub(crate) enum CliPuzzleCommand {
    /// Lists all puzzles
    #[default]
    List,
    /// Updates names for all puzzles based on HSC2 names
    UpdateNames,
}

impl CliPuzzleCommand {
    pub async fn execute(self, state: &AppState) -> eyre::Result<()> {
        let cli = state.get_cli_dummy_user().await?;

        match self {
            CliPuzzleCommand::List => {
                serde_json::to_writer_pretty(std::io::stdout(), &state.get_all_puzzles().await?)?;
            }
            CliPuzzleCommand::UpdateNames => {
                let puzzles = state.get_all_puzzles().await?;
                for mut puzzle_data in puzzles {
                    let old_name = puzzle_data.name.clone();
                    if let Some(hsc_id) = puzzle_data.hsc_id.clone() {
                        // IIFE to mimic try_block
                        let hsc_command_output =
                            async_process::Command::new(&*crate::env::HSC2_PATH)
                                .arg("puzzle")
                                .arg(&hsc_id)
                                .output()
                                .await;
                        let new_name = (|| {
                            Some(
                                serde_json::from_slice::<
                                    Vec<serde_json::Map<String, serde_json::Value>>,
                                >(&hsc_command_output.ok()?.stdout)
                                .ok()?
                                .first()?
                                .get("name")?
                                .as_str()?
                                .to_string(),
                            )
                        })();
                        if let Some(new_name) = new_name {
                            if new_name != *old_name {
                                puzzle_data.name = new_name.to_string();
                                state
                                    .update_puzzle(
                                        &cli,
                                        puzzle_data,
                                        "Renamed to match name in HSC2",
                                    )
                                    .await?;
                                println!("Renamed {hsc_id} from {old_name:?} to {new_name:?}");
                            } else {
                                println!("No change to {hsc_id} ({old_name:?})");
                            }
                        } else {
                            println!("Unable to find name for {hsc_id} ({old_name:?})");
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
