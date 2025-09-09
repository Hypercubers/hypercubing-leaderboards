use axum_typed_multipart::TryFromMultipart;

use crate::api::auth::{AuthConfirmAction, AuthContact, AuthType};
use crate::db::User;
use crate::html::otp::OtpResponse;
use crate::{AppError, AppState, RequestBody};

#[derive(TryFromMultipart)]
pub struct SignInEmailRequest {
    email: String,
    #[form_data(field_name = "cf-turnstile-response")]
    turnstile_response: Option<String>,
    redirect: Option<String>,
}
impl RequestBody for SignInEmailRequest {
    type Response = OtpResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        state.verify_turnstile(self.turnstile_response).await?;
        let account_exists = state.get_opt_user_from_email(&self.email).await?.is_some();
        let device_code = state
            .initiate_auth(
                AuthContact::Email(self.email),
                AuthConfirmAction::SignIn {
                    account_exists,
                    redirect: self.redirect,
                },
            )
            .await?;
        Ok(OtpResponse {
            user,
            device_code,
            auth_type: AuthType::EmailOtp,
        })
    }
}
