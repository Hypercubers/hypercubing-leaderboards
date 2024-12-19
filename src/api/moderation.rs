use crate::db::solve::SolveId;
use crate::db::user::User;
use crate::error::AppError;
use crate::AppState;
use crate::RequestBody;
use axum_typed_multipart::TryFromMultipart;

#[derive(TryFromMultipart)]
pub struct VerifySpeed {
    solve_id: i32,
}

pub struct VerifySpeedResponse {}

impl RequestBody for VerifySpeed {
    type Response = VerifySpeedResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let user = user.ok_or(AppError::NotLoggedIn)?;
        if !user.moderator {
            return Err(AppError::NotAuthorized);
        }

        let solve_id = SolveId(self.solve_id);
        state.verify_speed(solve_id, user.id).await?;

        Ok(VerifySpeedResponse {})
    }
}

#[poise::command(slash_command)]
pub async fn verify_speed(
    ctx: poise::Context<'_, AppState, AppError>,
    solve_id: i32,
) -> Result<(), AppError> {
    let request = VerifySpeed { solve_id };
    let state = ctx.data();
    let user = state
        .get_user_from_discord_id(ctx.author().id.into())
        .await?;
    let response = request.request(state.clone(), user).await?;
    ctx.send(response.into()).await?;

    Ok(())
}

impl From<VerifySpeedResponse> for poise::CreateReply {
    fn from(_val: VerifySpeedResponse) -> Self {
        poise::CreateReply::default().content("solve verified")
    }
}
