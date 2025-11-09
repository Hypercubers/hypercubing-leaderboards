use futures::StreamExt;

pub mod admin;
pub mod notify;
pub mod panic;
pub mod user;
pub mod verify;

use crate::{AppError, AppResult, AppState};

impl AppState {
    /// Returns a Discord user ID from Discord username.
    pub async fn discord_username_to_id(&self, username: &str) -> AppResult<u64> {
        let discord = self.try_discord()?;

        let mut user = None;
        for guild in discord.cache.guilds() {
            let stream = guild.members_iter(discord).filter_map(|member| async {
                let member = member.ok()?;
                username
                    .eq_ignore_ascii_case(&member.user.name)
                    .then_some(member.user.id)
            });
            let mut stream = Box::pin(stream);
            if let Some(u) = stream.next().await {
                if stream.next().await.is_some() {
                    // should be impossible
                    return Err(AppError::Other("Ambiguous Discord name".to_string()));
                }
                user = Some(u.get());
                break;
            }
        }
        user.ok_or(AppError::DiscordMemberNotFound)
    }
}
