use axum::response::IntoResponse;

use crate::db::{LogEntryDisplay, User};
use crate::traits::RequestBody;
use crate::{AppError, AppState};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct AuditLogPage {}

pub struct AuditLogPageResponse {
    user: Option<User>,
    log_entries: Vec<LogEntryDisplay>,
}

impl RequestBody for AuditLogPage {
    type Response = AuditLogPageResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        if !user.as_ref().ok_or(AppError::NotLoggedIn)?.moderator {
            return Err(AppError::NotAuthorized);
        }

        let log_entries = state.get_all_general_log_entries().await?;

        Ok(AuditLogPageResponse { user, log_entries })
    }
}

impl IntoResponse for AuditLogPageResponse {
    fn into_response(self) -> axum::response::Response {
        crate::render_html_template(
            "audit-log/general.html",
            &self.user,
            serde_json::json!({ "log_entries": self.log_entries }),
        )
    }
}
