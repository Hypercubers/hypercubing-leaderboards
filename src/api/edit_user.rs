use axum::response::{IntoResponse, Redirect, Response};
use axum_typed_multipart::TryFromMultipart;

use crate::db::{User, UserId};
use crate::{AppError, AppState, RequestBody};

#[derive(TryFromMultipart)]
pub struct UpdateUserDiscordIdRequest {
    pub target_user_id: i32,
    pub new_discord_id: Option<u64>,
}
impl RequestBody for UpdateUserDiscordIdRequest {
    type Response = UpdateUserDiscordIdResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let editor = user.ok_or(AppError::NotLoggedIn)?;
        if !editor.moderator {
            return Err(AppError::NotAuthorized); // must send OTP to new Discord account
        }
        let target = UserId(self.target_user_id);
        let new_discord_id = self.new_discord_id.filter(|&id| id != 0);
        state
            .update_user_discord_id(&editor, target, new_discord_id)
            .await?;

        Ok(UpdateUserDiscordIdResponse {
            target_user_id: target,
            new_discord_id: self.new_discord_id,
        })
    }
}

#[must_use]
#[derive(serde::Serialize)]
pub struct UpdateUserDiscordIdResponse {
    pub target_user_id: UserId,
    pub new_discord_id: Option<u64>,
}
impl IntoResponse for UpdateUserDiscordIdResponse {
    fn into_response(self) -> Response {
        Redirect::to(&self.target_user_id.relative_url()).into_response()
    }
}

pub struct UpdateUserEmailRequest {
    pub target_user_id: i32,
    pub new_email: Option<String>,
}
impl RequestBody for UpdateUserEmailRequest {
    type Response = UpdateUserEmailResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let editor = user.ok_or(AppError::NotLoggedIn)?;
        if !editor.moderator {
            return Err(AppError::NotAuthorized); // must send OTP to new Discord account
        }
        let target = UserId(self.target_user_id);
        let new_email = self.new_email.filter(|s| !s.is_empty());
        state
            .update_user_email(&editor, target, new_email.clone())
            .await?;

        Ok(UpdateUserEmailResponse {
            target_user_id: target,
            new_email,
        })
    }
}

#[must_use]
#[derive(serde::Serialize)]
pub struct UpdateUserEmailResponse {
    pub target_user_id: UserId,
    pub new_email: Option<String>,
}
impl IntoResponse for UpdateUserEmailResponse {
    fn into_response(self) -> Response {
        Redirect::to(&self.target_user_id.relative_url()).into_response()
    }
}

#[derive(TryFromMultipart)]
pub struct UpdateUserNameRequest {
    pub target_user_id: Option<i32>,
    pub new_name: Option<String>,
    pub redirect: Option<String>,
}
impl RequestBody for UpdateUserNameRequest {
    type Response = UpdateUserNameResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let editor = user.ok_or(AppError::NotLoggedIn)?;
        let target = UserId(self.target_user_id.unwrap_or(editor.id.0));
        let new_name = self.new_name.filter(|s| !s.is_empty());
        state
            .update_user_name(&editor, target, new_name.clone())
            .await?;

        Ok(UpdateUserNameResponse {
            target_user_id: target,
            new_name,
            redirect: self.redirect,
        })
    }
}

#[must_use]
#[derive(serde::Serialize)]
pub struct UpdateUserNameResponse {
    pub target_user_id: UserId,
    pub new_name: Option<String>,
    pub redirect: Option<String>,
}
impl IntoResponse for UpdateUserNameResponse {
    fn into_response(self) -> Response {
        Redirect::to(&self.redirect.unwrap_or(self.target_user_id.relative_url())).into_response()
    }
}
