/// Hyperspeedcube leaderboards server.
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(clap::Subcommand, Debug, Default)]
pub(crate) enum Command {
    /// Runs the database
    #[default]
    Run,
    /// Resets the database
    Reset,
    /// Migrates the database to the latest schema
    Migrate,
    /// Initializes the database from solves.csv (default)
    Init,
}
