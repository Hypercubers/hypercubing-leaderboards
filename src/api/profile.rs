use axum::body::Body;
use axum::response::{IntoResponse, Redirect, Response};
use axum_typed_multipart::TryFromMultipart;

use crate::db::EditAuthorization;
use crate::db::{PublicUser, User, UserId};
use crate::error::AppError;
use crate::{AppState, RequestBody};

#[derive(TryFromMultipart)]
pub struct UpdateProfile {
    user_id: i32,
    display_name: Option<String>,
}

pub struct UpdateProfileResponse {
    user_id: UserId,
    updated: bool,
}

#[poise::command(slash_command)]
pub async fn update_profile(
    ctx: poise::Context<'_, AppState, AppError>,
    display_name: Option<String>,
    target_user_id: Option<i32>,
) -> Result<(), AppError> {
    let state = ctx.data();
    let user = state
        .get_user_from_discord_id(ctx.author().id.into())
        .await?;

    let target_user_id = target_user_id.unwrap_or(user.clone().ok_or(AppError::NotLoggedIn)?.id.0);

    let request = UpdateProfile {
        user_id: target_user_id,
        display_name,
    };

    let response = request.request(state.clone(), user).await?;
    ctx.send(response.into()).await?;

    Ok(())
}

impl IntoResponse for UpdateProfileResponse {
    fn into_response(self) -> Response<Body> {
        Redirect::to(&format!("/solver?id={}", self.user_id.0)).into_response()
    }
}

impl From<UpdateProfileResponse> for poise::CreateReply {
    fn from(val: UpdateProfileResponse) -> Self {
        poise::CreateReply::default().content(if val.updated {
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
        let target_user_id = UserId(self.user_id);
        let auth = PublicUser::can_edit_id(target_user_id, &user).ok_or(AppError::NotAuthorized)?;

        match auth {
            EditAuthorization::Moderator => {
                tracing::info!(
                    editor_id = ?user.id,
                    ?target_user_id,
                    "modifying display_name as moderator"
                );
            }

            EditAuthorization::IsSelf => {
                tracing::info!(
                    editor_id = ?user.id,
                    ?target_user_id,
                    "modifying own display_name"
                );
            }
        }

        if self.display_name.is_some() {
            state
                .update_display_name(target_user_id, self.display_name)
                .await?;
            updated = true;
        }

        Ok(UpdateProfileResponse {
            user_id: target_user_id,
            updated,
        })
    }
}
