use crate::db::user::User;
use crate::error::AppError;
use crate::AppState;
use crate::RequestBody;
use axum_typed_multipart::TryFromMultipart;
use tokio::time::Duration;

//const WAIT_TIME: Duration = Duration::from_secs(5 * 60);
const WAIT_TIME: Duration = Duration::from_secs(10); // debug value

#[derive(serde::Deserialize, TryFromMultipart)]
pub struct VerifySpeedEvidence {
    speed_evidence_id: i32,
    verified: bool,
}

#[derive(serde::Deserialize)]
pub struct VerifySpeedEvidenceResponse {
    verified: bool,
}

impl RequestBody for VerifySpeedEvidence {
    type Response = VerifySpeedEvidenceResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let user = user.ok_or(AppError::NotLoggedIn)?;
        if !user.moderator {
            return Err(AppError::NotAuthorized);
        }

        state
            .verify_speed_evidence(self.speed_evidence_id, self.verified, user.id)
            .await?;

        Ok(VerifySpeedEvidenceResponse {
            verified: self.verified,
        })
    }
}

#[poise::command(slash_command)]
pub async fn verify_speed_evidence(
    ctx: poise::Context<'_, AppState, AppError>,
    speed_evidence_id: i32,
    verified: bool,
) -> Result<(), AppError> {
    let request = VerifySpeedEvidence {
        speed_evidence_id,
        verified,
    };
    let state = ctx.data();
    let user = state
        .get_user_from_discord_id(ctx.author().id.into())
        .await?;
    let response = request.request(state.clone(), user).await?;
    ctx.send(response.into()).await?;

    Ok(())
}

impl From<VerifySpeedEvidenceResponse> for poise::CreateReply {
    fn from(val: VerifySpeedEvidenceResponse) -> Self {
        poise::CreateReply::default().content(if val.verified {
            "solve verified"
        } else {
            "solve unverified"
        })
    }
}
