use axum_typed_multipart::TryFromMultipart;
use futures::StreamExt;
use tokio::time::Duration;

use crate::api::auth::TokenReturn;
use crate::db::User;
use crate::error::AppError;
use crate::{AppState, RequestBody};

#[cfg(not(debug_assertions))]
const WAIT_TIME: Duration = Duration::from_secs(5 * 60); // 5*60 seconds does not seem to work
#[cfg(debug_assertions)]
const WAIT_TIME: Duration = Duration::from_secs(10); // debug value

#[derive(TryFromMultipart)]
pub struct SignInDiscordForm {
    username: String,
    display_name: Option<String>,
    redirect: Option<String>,
    #[form_data(field_name = "cf-turnstile-response")]
    turnstile_response: Option<String>,
}

/// Sends a verification request via Discord DM to a user and returns the
/// Discord ID if successful.
async fn verify_discord(state: &AppState, username: &str) -> Result<i64, AppError> {
    use poise::serenity_prelude::*;

    let Some(discord) = &state.discord else {
        return Err(AppError::NoDiscord);
    };

    let mut user = None;
    for guild_id in discord.cache.guilds() {
        let stream = guild_id.members_iter(discord).filter_map(|member| async {
            let member = member.ok()?;
            tracing::debug!(?member.user.name, "user found");
            (member.user.name.eq_ignore_ascii_case(username)).then_some(member.user.id)
        });
        let mut stream = Box::pin(stream);
        if let Some(member) = stream.next().await {
            user = Some(member);
            break;
        }
    }
    let user: UserId = user.ok_or(AppError::UserDoesNotExist)?;

    let user_dms = user.create_dm_channel(discord).await?;

    let verify_id = "verify".to_string();

    let builder = CreateMessage::new()
        .embed(
            CreateEmbed::new()
                .title("Log in to the Hypercubing Leaderboards")
                .description("If you did not attempt to log in, ignore this message"),
        )
        .components(vec![CreateActionRow::Buttons(vec![CreateButton::new(
            verify_id.clone(),
        )
        .label("Yes, log me in")
        .style(ButtonStyle::Success)])]);
    let message = user_dms.send_message(discord, builder).await?;
    let collector = message
        .await_component_interaction(discord)
        .timeout(WAIT_TIME)
        .custom_ids(vec![verify_id]); // there shouldn't be any other ids

    // Wait for user interaction
    let interaction = collector
        .next()
        .await
        .ok_or(AppError::VerificationTimeout)?;

    interaction
        .create_response(discord, CreateInteractionResponse::Acknowledge)
        .await?;

    interaction
        .edit_response(
            discord,
            EditInteractionResponse::new().components(vec![CreateActionRow::Buttons(vec![
                CreateButton::new("a")
                    .label("Verified")
                    .style(ButtonStyle::Success)
                    .disabled(true),
            ])]),
        )
        .await?;

    Ok(user.into())
}

impl RequestBody for SignInDiscordForm {
    type Response = TokenReturn;

    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        state.verify_turnstile(self.turnstile_response).await?;

        let discord_id = tokio::time::timeout(WAIT_TIME, verify_discord(&state, &self.username))
            .await
            .map_err(|_| AppError::VerificationTimeout)??;
        let user = state.get_user_from_discord_id(discord_id).await?;

        let user = match user {
            Some(user) => user,
            None => {
                state
                    .create_user_discord(discord_id, self.display_name)
                    .await?
            }
        };

        let token = state.create_token(user.id).await?;

        Ok(TokenReturn {
            user,
            token: token.token,
            redirect: self.redirect,
        })
    }
}
