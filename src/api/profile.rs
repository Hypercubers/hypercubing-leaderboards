use crate::db::user::User;
use crate::error::AppError;
use crate::traits::RequestResponse;
use crate::AppState;
use crate::RequestBody;
use axum::response::IntoResponse;
use axum_typed_multipart::TryFromMultipart;

#[derive(serde::Deserialize, TryFromMultipart)]
pub struct UpdateProfile {
    display_name: Option<String>,
}

pub struct UpdateProfileResponse {
    updated: bool,
}

impl RequestResponse for UpdateProfileResponse {
    async fn as_axum_response(self) -> impl IntoResponse {
        if self.updated {
            "ok"
        } else {
            "no updates performed"
        }
    }
}

impl RequestBody for UpdateProfile {
    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<impl RequestResponse, AppError> {
        let user = user.ok_or(AppError::NotLoggedIn)?;
        let mut updated = false;

        if self.display_name.is_some() {
            state
                .update_display_name(user.id, self.display_name)
                .await?;
            updated = true;
        }

        Ok(UpdateProfileResponse { updated })
    }
}
