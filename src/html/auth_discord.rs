use crate::api::auth::TokenReturn;
use crate::db::user::User;
use crate::error::AppError;
use crate::traits::RequestResponse;
use crate::util::wait_for_none;
use crate::AppState;
use crate::RequestBody;
use axum_typed_multipart::TryFromMultipart;
use tokio::time::Duration;

//const WAIT_TIME: Duration = Duration::from_secs(5 * 60);
const WAIT_TIME: Duration = Duration::from_secs(10); // debug value

#[derive(serde::Deserialize, TryFromMultipart)]
pub struct SignInDiscordForm {
    username: String,
}

// None represents both invalid username and discord error
async fn verify_discord(state: &AppState, username: &str) -> Option<i32> {
    use poise::serenity_prelude::*;

    let user = UserId::new(186553034439000064); // me;
    let user_dms = user
        .create_dm_channel(state.discord_http.clone())
        .await
        .unwrap();
    // account_dms.send_message(_, CreateMessage::new().content("test"));
    /*account_dms
    .say(state.discord_http.clone(), "test")
    .await
    .unwrap();*/

    let builder = CreateMessage::new()
        .embed(
            CreateEmbed::new()
                .title("Please verify login to the hypercubers.xyz leaderboard.")
                .description("If you did not attempt to log in, please ignore this message"),
        )
        .components(vec![CreateActionRow::Buttons(vec![CreateButton::new(
            "verify",
        )
        .label("Verify")
        .style(ButtonStyle::Success)])]);
    let message = user_dms
        .send_message(state.discord_http.clone(), builder)
        .await
        .ok()?;
    let collector = message
        .await_component_interaction(state.discord_shard.clone())
        .timeout(WAIT_TIME);

    collector.next().await.map(|out| {
        dbg!(out);
        28
    })

    //Some(28) // me
}

impl RequestBody for SignInDiscordForm {
    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<impl RequestResponse, AppError> {
        let user_id = wait_for_none(verify_discord(&state, &self.username), WAIT_TIME)
            .await
            .ok_or(AppError::InvalidDiscordAccount)?;

        let token = state.create_token(user_id).await?;

        Ok(TokenReturn { token: token.token })
    }
}
