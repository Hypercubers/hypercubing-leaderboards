use axum::response::IntoResponse;

use crate::db::User;
use crate::traits::RequestBody;
use crate::{AppError, AppState};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct UsersPage {}

pub struct UsersPageResponse {
    user: Option<User>,
    users: Vec<User>,
}

impl RequestBody for UsersPage {
    type Response = UsersPageResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        if !user.as_ref().ok_or(AppError::NotLoggedIn)?.moderator {
            return Err(AppError::NotAuthorized);
        }

        let mut users = state.get_all_users().await?;

        users.sort_by_key(|u| u.id);

        Ok(UsersPageResponse { user, users })
    }
}

impl IntoResponse for UsersPageResponse {
    fn into_response(self) -> axum::response::Response {
        crate::render_html_template(
            "users.html",
            &self.user,
            serde_json::json!({ "users": self.users }),
        )
    }
}
