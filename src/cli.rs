/// Hyperspeedcube leaderboards server.
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(clap::Subcommand, Debug, Default)]
pub(crate) enum Command {
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
        user_command: UserCommand,
    },
}

#[derive(clap::Subcommand, Debug, Default)]
pub(crate) enum UserCommand {
    /// Lists all users
    #[default]
    List,
    /// Shows info for a particular user
    Info { user_id: i32 },
    /// Sets or clears the Discord ID of a user
    SetDiscord {
        user_id: i32,
        discord_id: Option<i64>,
    },
    /// Sets or clears the email address of a user
    SetEmail { user_id: i32, email: Option<String> },
    /// Promotes a user to a moderator
    Promote { user_id: i32 },
    /// Demotes a moderator
    Demote { user_id: i32 },
}
