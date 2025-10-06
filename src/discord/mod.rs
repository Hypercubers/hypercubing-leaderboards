use futures::StreamExt;

pub mod admin;
pub mod block;
pub mod notify;
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
                (member.user.name.eq_ignore_ascii_case(username)
                    || member
                        .nick
                        .is_some_and(|nick| nick.eq_ignore_ascii_case(username)))
                .then_some(member.user.id)
            });
            let mut stream = Box::pin(stream);
            if let Some(u) = stream.next().await {
                if stream.next().await.is_some() {
                    return Err(AppError::Other("Ambiguous Discord name".to_string()));
                }
                user = Some(u.get());
                break;
            }
        }
        user.ok_or(AppError::DiscordMemberNotFound)
    }
}
