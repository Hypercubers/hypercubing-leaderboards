use serenity::all::ChannelId;

lazy_static! {
    /// Logging configuration.
    pub static ref RUST_LOG: String =
        dotenvy::var("RUST_LOG").expect("missing RUST_LOG environment variable");

    /// Domain name, with no trailing slash. Example: `https://lb.hypercubing.xyz`
    pub static ref DOMAIN: String = dotenvy::var("DOMAIN_NAME")
        .expect("missing DOMAIN_NAME environment variable")
        .trim_end_matches('/')
        .to_string();

    /// Discord bot token.
    pub static ref DISCORD_TOKEN: String =
        dotenvy::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN environment variable");

    /// Discord channel for leaderboard verification requests.
    pub static ref VERIFICATION_CHANNEL: ChannelId = dotenvy::var("VERIFICATION_CHANNEL_ID")
        .expect("missing VERIFICATION_CHANNEL_ID environment variable")
        .parse()
        .expect("invalid value for VERIFICATION_CHANNEL_ID");

    /// Discord channel for leaderboard updates.
    pub static ref UPDATE_CHANNEL: ChannelId = dotenvy::var("UPDATE_CHANNEL_ID")
        .expect("missing UPDATE_CHANNEL_ID environment variable")
        .parse()
        .expect("invalid value for UPDATE_CHANNEL_ID");
}
