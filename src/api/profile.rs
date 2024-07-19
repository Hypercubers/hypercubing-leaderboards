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

#[poise::command(slash_command)]
pub async fn update_profile(
    ctx: poise::Context<'_, AppState, AppError>,
    display_name: Option<String>,
) -> Result<(), AppError> {
    let request = UpdateProfile { display_name };
    let state = ctx.data();
    let user = state
        .get_user_from_discord_id(ctx.author().id.into())
        .await?;
    let response = request.request(state.clone(), user).await?;
    ctx.send(response.into()).await?;

    Ok(())
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

impl Into<poise::CreateReply> for UpdateProfileResponse {
    fn into(self) -> poise::CreateReply {
        poise::CreateReply::default().content(if self.updated {
            "ok"
        } else {
            "no updates performed"
        })
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
