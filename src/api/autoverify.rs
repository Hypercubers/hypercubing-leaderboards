use axum_typed_multipart::TryFromMultipart;

use crate::api::UpdateSolveResponse;
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
        state.enqueue_solve_to_autoverify(solve_id).await?;
        Ok(UpdateSolveResponse { solve_id })
    }
}
