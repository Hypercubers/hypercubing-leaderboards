use axum::response::IntoResponse;

use crate::db::User;
use crate::traits::RequestBody;
use crate::{AppError, AppState};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SettingsPage {}

pub struct SettingsPageResponse {
    user: Option<User>,
}

impl RequestBody for SettingsPage {
    type Response = SettingsPageResponse;

    async fn request(
        self,
        _state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        if user.is_none() {
            return Err(AppError::NotLoggedIn);
        }
        Ok(SettingsPageResponse { user })
    }
}

impl IntoResponse for SettingsPageResponse {
    fn into_response(self) -> axum::response::Response {
        crate::render_html_template("settings.html", &self.user, serde_json::json!({}))
    }
}
