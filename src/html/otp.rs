use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use axum_typed_multipart::TryFromMultipart;

use crate::api::auth::{AuthConfirmResponse, AuthType};
use crate::db::User;
use crate::traits::RequestBody;
use crate::{AppError, AppState};

pub struct OtpResponse {
    pub user: Option<User>,
    pub device_code: String,
    pub auth_type: AuthType,
}
impl IntoResponse for OtpResponse {
    fn into_response(self) -> Response {
        crate::render_html_template(
            "submit-otp.html",
            &self.user,
            serde_json::json!({
                "device_code": self.device_code,
                "try_again_link": "/sign-in",
                "support_email": *crate::env::SUPPORT_EMAIL,
                "check_discord_dms": self.auth_type == AuthType::DiscordOtp,
                "check_email": self.auth_type == AuthType::EmailOtp,
            }),
        )
    }
}

#[derive(TryFromMultipart)]
pub struct SubmitOtpRequest {
    device_code: String,
    otp: String,
}
impl RequestBody for SubmitOtpRequest {
    type Response = AuthConfirmResponse;

    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        state.confirm_otp(&self.device_code, &self.otp).await
    }
}

// TODO: impl is far from type definition
impl IntoResponse for AuthConfirmResponse {
    fn into_response(self) -> Response {
        let mut jar = CookieJar::new();
        if let Some(token_string) = self.token_string {
            jar = jar.add(
                Cookie::build(("token", token_string))
                    .http_only(true)
                    .secure(true)
                    .same_site(SameSite::Strict),
            );
        }

        // assume the query parameter is a relative url, which if js/form.js is doing its job will be
        (jar, Redirect::to(&self.redirect)).into_response()
    }
}
