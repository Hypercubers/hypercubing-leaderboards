use axum::response::IntoResponse;

use crate::db::{RenderedAuditLogEntry, SolveId, User, UserId};
use crate::traits::{Linkable, RequestBody};
use crate::{AppError, AppState};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct GeneralAuditLogPage {}

pub struct GeneralAuditLogPageResponse {
    user: Option<User>,
    log_entries: Vec<RenderedAuditLogEntry>,
}

impl RequestBody for GeneralAuditLogPage {
    type Response = GeneralAuditLogPageResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        if !user.as_ref().ok_or(AppError::NotLoggedIn)?.moderator {
            return Err(AppError::NotAuthorized);
        }

        let log_entries = state
            .get_all_general_log_entries()
            .await?
            .into_iter()
            .map(|entry| entry.display_full())
            .collect();

        Ok(GeneralAuditLogPageResponse { user, log_entries })
    }
}

impl IntoResponse for GeneralAuditLogPageResponse {
    fn into_response(self) -> axum::response::Response {
        crate::render_html_template(
            "audit-log/general.html",
            &self.user,
            serde_json::json!({ "log_entries": self.log_entries }),
        )
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SolveAuditLogPage {
    id: SolveId,
}

pub struct SolveAuditLogPageResponse {
    user: Option<User>,
    solve_id: SolveId,
    log_entries: Vec<RenderedAuditLogEntry>,
}

impl RequestBody for SolveAuditLogPage {
    type Response = SolveAuditLogPageResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        if !user.as_ref().ok_or(AppError::NotLoggedIn)?.moderator {
            return Err(AppError::NotAuthorized);
        }

        let log_entries = state
            .get_all_solve_log_entries(self.id)
            .await?
            .into_iter()
            .map(|entry| entry.display_full())
            .collect();

        Ok(SolveAuditLogPageResponse {
            user,
            solve_id: self.id,
            log_entries,
        })
    }
}

impl IntoResponse for SolveAuditLogPageResponse {
    fn into_response(self) -> axum::response::Response {
        crate::render_html_template(
            "audit-log/solve.html",
            &self.user,
            serde_json::json!({
                "solve_id": self.solve_id,
                "solve_url": self.solve_id.relative_url(),
                "log_entries": self.log_entries,
            }),
        )
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct UserAuditLogPage {
    id: UserId,
}

pub struct UserAuditLogPageResponse {
    user: Option<User>,
    target_user: User,
    log_entries: Vec<RenderedAuditLogEntry>,
}

impl RequestBody for UserAuditLogPage {
    type Response = UserAuditLogPageResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        if !user.as_ref().ok_or(AppError::NotLoggedIn)?.moderator {
            return Err(AppError::NotAuthorized);
        }

        let target_user = state.get_user(self.id).await?;
        let log_entries = state
            .get_all_user_log_entries(self.id)
            .await?
            .into_iter()
            .map(|entry| entry.display_full())
            .collect();

        Ok(UserAuditLogPageResponse {
            user,
            target_user,
            log_entries,
        })
    }
}

impl IntoResponse for UserAuditLogPageResponse {
    fn into_response(self) -> axum::response::Response {
        let target_user_name = self.target_user.to_public().display_name();
        crate::render_html_template(
            "audit-log/user.html",
            &self.user,
            serde_json::json!({
                "target_user": self.target_user,
                "target_user_name": target_user_name,
                "log_entries": self.log_entries,
            }),
        )
    }
}
