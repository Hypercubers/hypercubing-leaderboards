use axum_typed_multipart::TryFromMultipart;

use crate::api::auth::{AuthConfirmAction, AuthContact, AuthType};
use crate::db::User;
use crate::html::otp::OtpResponse;
use crate::{AppError, AppState, RequestBody};

#[derive(TryFromMultipart)]
pub struct SignInDiscordRequest {
    username: String,
    #[form_data(field_name = "cf-turnstile-response")]
    turnstile_response: Option<String>,
    redirect: Option<String>,
}
impl RequestBody for SignInDiscordRequest {
    type Response = OtpResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let discord_id = state.discord_username_to_id(&self.username).await?;
        state.verify_turnstile(self.turnstile_response).await?; // verify after checking that username is correct
        let account_exists = state
            .get_opt_user_from_discord_id(discord_id)
            .await?
            .is_some();
        let device_code = state
            .initiate_auth(
                AuthContact::Discord(discord_id),
                AuthConfirmAction::SignIn {
                    account_exists,
                    redirect: self.redirect,
                },
            )
            .await?;
        Ok(OtpResponse {
            user,
            device_code,
            auth_type: AuthType::DiscordOtp,
        })
    }
}
