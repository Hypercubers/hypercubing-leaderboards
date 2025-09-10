use std::io::Write;

use crate::db::UserId;
use crate::AppState;

/// Hyperspeedcube leaderboards server.
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct CliArgs {
    #[command(subcommand)]
    pub command: Option<CliCommand>,
}

async fn confirm_yn(default: bool) -> bool {
    match default {
        true => print!("[Y/n] "),
        false => print!("[y/N] "),
    };
    std::io::stdout().flush().expect("error flushing stdout");
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("error reading from stdin");
    match buf.trim().to_ascii_lowercase().as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => default,
    }
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
}

impl CliCommand {
    pub async fn execute(self, state: AppState) -> eyre::Result<()> {
        match self {
            CliCommand::Run => {
                crate::run_web_server(state).await;
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
                println!("Set user Discord ID for user #{user_id} to {discord_id:?}")
            }
            CliUserCommand::SetEmail { user_id, email } => {
                let target = UserId(user_id);
                state.update_user_email(&cli, target, email.clone()).await?;
                println!("Set user email for user #{user_id} to {email:?}")
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
