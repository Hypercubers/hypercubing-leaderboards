use crate::db::user::User;
use crate::error::AppError;
use crate::AppState;
use crate::RequestBody;
use axum::body::Body;
use axum::response::IntoResponse;
use axum::response::Response;
use axum_typed_multipart::TryFromMultipart;

#[derive(serde::Deserialize, TryFromMultipart)]
pub struct UpdateProfile {
    display_name: Option<String>,
}

pub struct UpdateProfileResponse {
    updated: bool,
}

impl IntoResponse for UpdateProfileResponse {
    fn into_response(self) -> Response<Body> {
        if self.updated {
            "ok"
        } else {
            "no updates performed"
        }
        .into_response()
    }
}

impl RequestBody for UpdateProfile {
    type Response = UpdateProfileResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
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
