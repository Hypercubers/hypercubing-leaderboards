use axum_typed_multipart::TryFromMultipart;

use crate::api::{UpdatePendingSubmissionsResponse, UpdateSolveResponse};
use crate::db::{SolveId, User};
use crate::{AppError, AppState, RequestBody};

#[derive(Debug, TryFromMultipart)]
pub struct RequestAutoVerifySolve {
    pub solve_id: i32,
}
impl RequestBody for RequestAutoVerifySolve {
    type Response = UpdateSolveResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let editor = user.ok_or(AppError::NotLoggedIn)?;
        if !editor.moderator {
            return Err(AppError::NotAuthorized);
        }
        let solve_id = SolveId(self.solve_id);
        state.autoverifier.enqueue(solve_id).await;
        Ok(UpdateSolveResponse { solve_id })
    }
}

#[derive(Debug, TryFromMultipart)]
pub struct RequestAutoVerifyAllSolves {}
impl RequestBody for RequestAutoVerifyAllSolves {
    type Response = UpdatePendingSubmissionsResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let editor = user.ok_or(AppError::NotLoggedIn)?;
        if !editor.moderator {
            return Err(AppError::NotAuthorized);
        }
        for solve in state.get_pending_submissions().await?.into_iter() {
            state.autoverifier.enqueue(solve.id).await;
        }
        Ok(UpdatePendingSubmissionsResponse)
    }
}
