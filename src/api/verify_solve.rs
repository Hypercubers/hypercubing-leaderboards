use axum_typed_multipart::{TryFromField, TryFromMultipart};

use crate::api::UpdateSolveResponse;
use crate::db::{SolveId, User};
use crate::{AppError, AppState, RequestBody};

#[derive(TryFromMultipart)]
pub struct VerifySolveRequest {
    solve_id: i32,
    speed: Option<VerifyAction>,
    fmc: Option<VerifyAction>,
    audit_log_comment: Option<String>,
}

#[derive(TryFromField, Debug)]
#[try_from_field(rename_all = "snake_case")]
enum VerifyAction {
    Accept,
    Reject,
    Unverify,
}
impl From<VerifyAction> for Option<bool> {
    fn from(value: VerifyAction) -> Self {
        match value {
            VerifyAction::Accept => Some(true),
            VerifyAction::Reject => Some(false),
            VerifyAction::Unverify => None,
        }
    }
}

impl RequestBody for VerifySolveRequest {
    type Response = UpdateSolveResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let editor = user.ok_or(AppError::NotLoggedIn)?;

        let audit_log_comment = self.audit_log_comment.unwrap_or_default();

        if let Some(speed_verify) = self.speed {
            state
                .verify_speed(
                    &editor,
                    SolveId(self.solve_id),
                    speed_verify.into(),
                    &audit_log_comment,
                )
                .await?;
        }

        if let Some(fmc_verify) = self.fmc {
            state
                .verify_fmc(
                    &editor,
                    SolveId(self.solve_id),
                    fmc_verify.into(),
                    &audit_log_comment,
                )
                .await?;
        }

        Ok(UpdateSolveResponse {
            solve_id: SolveId(self.solve_id),
        })
    }
}
