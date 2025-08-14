use serenity::all::ChannelId;

fn get_env_var(name: &str) -> String {
    dotenvy::var(name).expect(&format!("missing {name} environment variable"))
}

fn parse_env_var<T: std::str::FromStr>(name: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    get_env_var(name)
        .parse()
        .expect(&format!("invalid value for {name} environment variable"))
}

lazy_static! {
    /// Logging configuration.
    pub static ref RUST_LOG: String = get_env_var("RUST_LOG");

    /// Domain name, with no trailing slash. Example: `https://lb.hypercubing.xyz`
    pub static ref DOMAIN_NAME: String =
        get_env_var("DOMAIN_NAME").trim_end_matches('/').to_string();

    /// Database URL. Example: `postgres://leaderboards_bot:password@localhost/leaderboards`
    pub static ref DATABASE_URL: String = get_env_var("DATABASE_URL");

    /// Discord bot token.
    pub static ref DISCORD_TOKEN: String = get_env_var("DISCORD_TOKEN");
    /// Discord channel for leaderboard verification requests.
    pub static ref PRIVATE_UPDATES_CHANNEL_ID: ChannelId = parse_env_var("PRIVATE_UPDATES_CHANNEL_ID");
    /// Discord channel for leaderboard updates.
    pub static ref PUBLIC_UPDATES_CHANNEL_ID: ChannelId = parse_env_var("PUBLIC_UPDATES_CHANNEL_ID");

    /// Email host URL.
    pub static ref SMTP_HOST: String = get_env_var("SMTP_HOST");
    /// Email host port.
    pub static ref SMTP_HOST_PORT: u16 = parse_env_var("SMTP_HOST_PORT");
    /// SMTP auth username.
    pub static ref SMTP_USERNAME: String = get_env_var("SMTP_USERNAME");
    /// SMTP auth password.
    pub static ref SMTP_PASSWORD: String = get_env_var("SMTP_PASSWORD");
    /// Email "from" name.
    pub static ref SMTP_FROM_NAME: String = get_env_var("SMTP_FROM_NAME");
    /// Email "from" address.
    pub static ref SMTP_FROM_ADDRESS: String = get_env_var("SMTP_FROM_ADDRESS");
    /// Email address for users to request technical support.
    pub static ref SUPPORT_EMAIL: String = get_env_var("SUPPORT_EMAIL");

    /// Cloudflare Turnstile site key. This is public.
    pub static ref TURNSTILE_SITE_KEY: String = get_env_var("TURNSTILE_SITE_KEY");
    /// Cloudflare Turnstile secret key.
    pub static ref TURNSTILE_SECRET_KEY: String = get_env_var("TURNSTILE_SECRET_KEY");

}
