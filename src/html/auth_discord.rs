use crate::api::auth::TokenReturn;
use crate::db::user::User;
use crate::error::AppError;
use crate::util::wait_for_none;
use crate::AppState;
use crate::RequestBody;
use axum_typed_multipart::TryFromMultipart;
use futures::StreamExt;
use tokio::time::Duration;

#[cfg(not(debug_assertions))]
const WAIT_TIME: Duration = Duration::from_secs(1 * 60); // 5*60 seconds does not seem to work
#[cfg(debug_assertions)]
const WAIT_TIME: Duration = Duration::from_secs(10); // debug value

#[derive(TryFromMultipart)]
pub struct SignInDiscordForm {
    username: String,
    display_name: Option<String>,
    redirect: Option<String>,
}

// None represents both invalid username and discord error
/// provides discord id of username
async fn verify_discord(state: &AppState, username: &str) -> Option<i64> {
    use poise::serenity_prelude::*;

    let Some(discord) = &state.discord else {
        return None;
    };

    let mut user = None;
    for guild_id in discord.cache.guilds() {
        let stream = guild_id.members_iter(discord).filter_map(|member| async {
            let member = member.ok()?;
            println!("user {:?}", member.user.name);
            if member.user.name == username {
                Some(member.user.id)
            } else {
                None
            }
        });
        let mut stream = Box::pin(stream);
        if let Some(member) = stream.next().await {
            user = Some(member);
            break;
        }
    }
    let user: UserId = user?;

    let user_dms = user.create_dm_channel(discord).await.ok()?;

    let verify_id = "verify".to_string();

    let builder = CreateMessage::new()
        .embed(
            CreateEmbed::new()
                .title("Please verify login to the hypercubers.xyz leaderboard.")
                .description("If you did not attempt to log in, please ignore this message"),
        )
        .components(vec![CreateActionRow::Buttons(vec![CreateButton::new(
            verify_id.clone(),
        )
        .label("Verify")
        .style(ButtonStyle::Success)])]);
    let mut message = user_dms.send_message(discord, builder).await.ok()?;
    let collector = message
        .await_component_interaction(discord)
        .timeout(WAIT_TIME)
        .custom_ids(vec![verify_id]); // there shouldn't be any other ids
    let interaction = collector.next().await;

    if let Some(interaction) = interaction {
        message
            .edit(
                discord,
                EditMessage::new().components(vec![CreateActionRow::Buttons(vec![
                    CreateButton::new("a")
                        .label("Verified")
                        .style(ButtonStyle::Success)
                        .disabled(true),
                ])]),
            )
            .await
            .unwrap();

        let _ = interaction
            .create_response(discord, CreateInteractionResponse::Acknowledge)
            .await;

        Some(user.into())
    } else {
        None
    }
}

impl RequestBody for SignInDiscordForm {
    type Response = TokenReturn;

    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let discord_id = wait_for_none(verify_discord(&state, &self.username), WAIT_TIME)
            .await
            .ok_or(AppError::InvalidDiscordAccount)?;
        let user = state.get_user_from_discord_id(discord_id).await?;

        let user = match user {
            Some(user) => {
                /*if self.display_name.is_some() {
                    return Err(AppError::InvalidQuery("account already exists".to_string()));
                }*/
                // if an existing user signs in with the new user box, ignore the chosen display name
                user
            }
            None => {
                state
                    .create_user_discord(discord_id, self.display_name)
                    .await?
            }
        };

        let token = state.create_token(user.id).await?;

        Ok(TokenReturn {
            token: token.token,
            redirect: Some(self.redirect.unwrap_or(format!("/solver?id={}", user.id))),
        })
    }
}
