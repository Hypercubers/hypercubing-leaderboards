use axum::body::{Body, Bytes};
use axum::response::{IntoResponse, Redirect, Response};
use axum_typed_multipart::{FieldData, TryFromMultipart};
use chrono::NaiveDate;

use crate::db::{EditAuthorization, SolveId, User};
use crate::traits::RequestBody;
use crate::{AppError, AppState};

// pub struct SolveData {
//     log_file: String,
//     puzzle_hsc_id: String, // hsc puzzle id
//     move_count: i32,
//     uses_macros: bool,
//     uses_filters: bool,
//     speed_cs: Option<i32>,
//     memo_cs: Option<i32>,
//     blind: bool,
//     scramble_seed: Option<String>,
//     program_version: String, // hsc program version
//     valid_solve: bool,
// }

// fn verify_log(log_file: String) -> SolveData {
//     // dummy data
//     SolveData {
//         log_file,
//         puzzle_hsc_id: "3x3x3".to_string(), // hsc puzzle id
//         move_count: 100,
//         uses_macros: false,
//         uses_filters: false,
//         speed_cs: Some(1000),
//         memo_cs: None,
//         blind: false,
//         scramble_seed: None,
//         program_version: "2.0.0".to_string(), // hsc program version
//         valid_solve: true,
//     }
// }

// #[derive(TryFromMultipart)]
// pub struct UploadSolveRequest {
//     log_file: Option<String>,
//     #[serde(deserialize_with = "empty_string_as_none")]
//     video_url: Option<String>,
// }

// pub struct UploadSolveResponse {}

// impl RequestBody for UploadSolveRequest {
//     async fn request(
//         self,
//         state: AppState,
//         user: Option<User>,
//     ) -> Result<impl RequestResponse, AppError> {
//         let user = user.ok_or(AppError::NotLoggedIn)?;
//         let log_file = self.log_file.ok_or(AppError::NoLogFile)?;

//         let solve_data = verify_log(log_file);

//         let puzzle_id = query!(
//             "SELECT id
//             FROM Puzzle
//             WHERE Puzzle.hsc_id = $1",
//             solve_data.puzzle_hsc_id,
//         )
//         .fetch_optional(&state.pool)
//         .await?
//         .ok_or(AppError::PuzzleVersionDoesNotExist)?
//         .id;

//         let program_version_id = query!(
//             "SELECT ProgramVersion.id
//             FROM ProgramVersion
//             JOIN Program
//             ON Program.id = ProgramVersion.program_id
//             WHERE Program.name = 'Hyperspeedcube'
//             AND ProgramVersion.version = $1",
//             solve_data.program_version
//         )
//         .fetch_optional(&state.pool)
//         .await?
//         .ok_or(AppError::ProgramVersionDoesNotExist)?
//         .id;

//         let solve = query!(
//             "INSERT INTO Solve
//                 (log_file, user_id, puzzle_id, move_count,
//                 uses_macros, uses_filters, speed_cs, memo_cs,
//                 blind, scramble_seed, program_version_id, valid_solve)
//             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
//             RETURNING *",
//             solve_data.log_file,
//             user.id,
//             puzzle_id,
//             solve_data.move_count,
//             solve_data.uses_macros,
//             solve_data.uses_filters,
//             solve_data.speed_cs,
//             solve_data.memo_cs,
//             solve_data.blind,
//             solve_data.scramble_seed,
//             program_version_id,
//             solve_data.valid_solve
//         )
//         .fetch_optional(&state.pool)
//         .await?
//         .ok_or(AppError::CouldNotInsertSolve)?;

//         if self.video_url.is_some() {
//             query!(
//                 "INSERT INTO SpeedEvidence
//                 (solve_id, video_url)
//             VALUES ($1, $2)
//             RETURNING *",
//                 solve.id,
//                 self.video_url
//             )
//             .fetch_optional(&state.pool)
//             .await?
//             .ok_or(AppError::CouldNotInsertSolve)?;
//         }

//         Ok(UploadSolveResponse {})
//     }
// }

// impl RequestResponse for UploadSolveResponse {
//     async fn as_axum_response(self) -> impl IntoResponse {
//         "ok"
//     }
// }

#[derive(Debug, TryFromMultipart)]
pub struct ManualSubmitSolve {
    // Event
    pub puzzle_id: i32,
    pub variant_id: Option<i32>,
    pub program_id: Option<i32>,

    // Metadata
    pub solve_date: NaiveDate,
    pub notes: Option<String>,

    // Speedsolve
    pub solve_h: Option<i32>,
    pub solve_m: Option<i32>,
    pub solve_s: Option<i32>,
    pub solve_cs: Option<i32>,
    pub uses_filters: bool,
    pub uses_macros: bool,
    pub average: bool,
    pub one_handed: bool,
    pub blind: bool,
    pub memo_h: Option<i32>,
    pub memo_m: Option<i32>,
    pub memo_s: Option<i32>,
    pub memo_cs: Option<i32>,
    pub video_url: Option<String>,

    // Fewest moves
    pub move_count: Option<i32>,
    pub computer_assisted: bool,
    pub log_file: Option<FieldData<Bytes>>,
}

pub struct ManualSubmitSolveResponse {
    pub solve_id: SolveId,
}

impl RequestBody for ManualSubmitSolve {
    type Response = ManualSubmitSolveResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let user = user.ok_or(AppError::NotLoggedIn)?;

        if self.video_url.is_none() && self.log_file.is_none() {
            return Err(AppError::NoEvidence);
        }

        let solve_id = state.add_solve_external(user.id, self).await?;

        Ok(ManualSubmitSolveResponse { solve_id })
    }
}

impl IntoResponse for ManualSubmitSolveResponse {
    fn into_response(self) -> Response<Body> {
        Redirect::to(&format!("/solve?id={}", self.solve_id.0)).into_response()
    }
}

/// returns Ok(_) if authorized, Err(_) if not
async fn authorize_to_edit(
    solve_id: i32,
    state: &AppState,
    user: Option<&User>,
) -> Result<EditAuthorization, AppError> {
    let user = user.ok_or(AppError::NotLoggedIn)?;
    let solve = state
        .get_solve(SolveId(solve_id))
        .await?
        .ok_or(AppError::InvalidSolve)?;

    let auth = solve.can_edit(user).ok_or(AppError::NotAuthorized)?;

    match auth {
        EditAuthorization::Moderator => {
            tracing::info!(
                editor_id = ?user.id,
                ?solve_id,
                "modifying solve as moderator"
            );
        }

        EditAuthorization::IsSelf => {
            tracing::info!(editor_id = ?user.id, ?solve_id, "modifying own solve");
        }
    }

    Ok(auth)
}

pub struct UpdateSolveResponse {
    solve_id: i32,
}

impl IntoResponse for UpdateSolveResponse {
    fn into_response(self) -> Response<Body> {
        Redirect::to(&format!("/solve?id={}", self.solve_id)).into_response()
    }
}

macro_rules! impl_request_body {
    ($ty:ident, $update:ident) => {
        impl RequestBody for $ty {
            type Response = UpdateSolveResponse;

            async fn request(
                self,
                state: AppState,
                user: Option<User>,
            ) -> Result<Self::Response, AppError> {
                let edit_authorization =
                    authorize_to_edit(self.solve_id, &state, user.as_ref()).await?;

                state.$update(&self).await?;

                let solve_id = SolveId(self.solve_id);
                if matches!(edit_authorization, EditAuthorization::IsSelf) {
                    state.alert_discord_to_verify(solve_id, true).await;
                }

                Ok(UpdateSolveResponse {
                    solve_id: self.solve_id,
                })
            }
        }
    };
}

#[derive(Debug, TryFromMultipart, Clone)]
pub struct UpdateSolveVideoUrl {
    pub solve_id: i32,
    pub video_url: Option<String>,
}

impl_request_body!(UpdateSolveVideoUrl, update_video_url);

#[derive(Debug, TryFromMultipart, Clone)]
pub struct UpdateSolveSpeedCs {
    pub solve_id: i32,
    pub speed_cs: Option<i32>,
}

impl_request_body!(UpdateSolveSpeedCs, update_speed_cs);

#[derive(Debug, TryFromMultipart, Clone)]
pub struct UpdateSolveCategory {
    pub solve_id: i32,
    pub puzzle_id: i32,
    pub blind: bool,
    pub uses_filters: bool,
    pub uses_macros: bool,
    // TODO: rename these and add more fields
}

impl_request_body!(UpdateSolveCategory, update_solve_category);

// #[derive(Debug, TryFromMultipart, Clone)]
// pub struct UpdateSolveProgramVersionId {
//     pub solve_id: i32,
//     pub program_version_id: i32,
// }

// impl_request_body!(UpdateSolveProgramVersionId, update_solve_program_version_id);

#[derive(Debug, TryFromMultipart, Clone)]
pub struct UpdateSolveMoveCount {
    pub solve_id: i32,
    pub move_count: Option<i32>,
}

impl_request_body!(UpdateSolveMoveCount, update_move_count);

#[cfg(test)]
mod tests {
    // #[sqlx::test]
    // fn upload_successful(pool: PgPool) -> AppResult {
    //     let state = AppState {
    //         pool,
    //         otps: Default::default(),
    //         discord: None,
    //     };
    //     let user = state
    //         .create_user("user@example.com".to_string(), Some("user 1".to_string()))
    //         .await?;

    //     let puzzle_id = query!("INSERT INTO Puzzle (name) VALUES ('3x3x3') RETURNING id")
    //         .fetch_one(&state.pool)
    //         .await?
    //         .id;

    //     let program_id =
    //         query!("INSERT INTO Program (name, abbreviation) VALUES ('Hyperspeedcube', 'HSC') RETURNING id")
    //             .fetch_one(&state.pool)
    //             .await?
    //             .id;

    //     let program_version_id = query!(
    //         "INSERT INTO ProgramVersion (program_id, version) VALUES ($1, '2.0.0') RETURNING id",
    //         program_id
    //     )
    //     .fetch_one(&state.pool)
    //     .await?
    //     .id;

    //     UploadSolveExternal {
    //         puzzle_id,
    //         speed_cs: Some(1),
    //         blind: false,
    //         memo_cs: None,
    //         uses_filters: true,
    //         uses_macros: false,
    //         computer_assisted: false,
    //         video_url: Some("https://example.com".to_string()),
    //         program_version_id,
    //         move_count: Some(10000000),
    //         log_file: Some("dummy log file".to_string()),
    //     }
    //     .request(state, Some(user))
    //     .await?;

    //     Ok(())
    // }
}
