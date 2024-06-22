use crate::db::user::User;
use crate::error::AppError;
use crate::traits::{RequestBody, RequestResponse};
use crate::util::{empty_string_as_none, on_as_true};
use crate::AppState;
use axum::response::IntoResponse;
use axum_typed_multipart::TryFromMultipart;
use sqlx::query;

pub struct SolveData {
    log_file: String,
    puzzle_hsc_id: String, // hsc puzzle id
    move_count: i32,
    uses_macros: bool,
    uses_filters: bool,
    speed_cs: Option<i32>,
    memo_cs: Option<i32>,
    blind: bool,
    scramble_seed: Option<String>,
    program_version: String, // hsc program version
    valid_solve: bool,
}

fn verify_log(log_file: String) -> SolveData {
    // dummy data
    SolveData {
        log_file,
        puzzle_hsc_id: "3x3x3".to_string(), // hsc puzzle id
        move_count: 100,
        uses_macros: false,
        uses_filters: false,
        speed_cs: Some(1000),
        memo_cs: None,
        blind: false,
        scramble_seed: None,
        program_version: "2.0.0".to_string(), // hsc program version
        valid_solve: true,
    }
}

#[derive(serde::Deserialize, TryFromMultipart)]
pub struct UploadSolveRequest {
    log_file: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    video_url: Option<String>,
}

pub struct UploadSolveResponse {}

impl RequestBody for UploadSolveRequest {
    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<impl RequestResponse, AppError> {
        let user = user.ok_or(AppError::NotLoggedIn)?;
        let log_file = self.log_file.ok_or(AppError::NoLogFile)?;

        let solve_data = verify_log(log_file);

        let puzzle_id = query!(
            "SELECT id
            FROM Puzzle
            WHERE Puzzle.hsc_id = $1",
            solve_data.puzzle_hsc_id,
        )
        .fetch_optional(&state.pool)
        .await?
        .ok_or(AppError::PuzzleVersionDoesNotExist)?
        .id;

        let program_version_id = query!(
            "SELECT ProgramVersion.id
            FROM ProgramVersion
            JOIN Program
            ON Program.id = ProgramVersion.program_id
            WHERE Program.name = 'Hyperspeedcube'
            AND ProgramVersion.version = $1",
            solve_data.program_version
        )
        .fetch_optional(&state.pool)
        .await?
        .ok_or(AppError::ProgramVersionDoesNotExist)?
        .id;

        let solve = query!(
            "INSERT INTO Solve
                (log_file, user_id, puzzle_id, move_count,
                uses_macros, uses_filters, speed_cs, memo_cs,
                blind, scramble_seed, program_version_id, valid_solve) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *",
            solve_data.log_file,
            user.id,
            puzzle_id,
            solve_data.move_count,
            solve_data.uses_macros,
            solve_data.uses_filters,
            solve_data.speed_cs,
            solve_data.memo_cs,
            solve_data.blind,
            solve_data.scramble_seed,
            program_version_id,
            solve_data.valid_solve
        )
        .fetch_optional(&state.pool)
        .await?
        .ok_or(AppError::CouldNotInsertSolve)?;

        if self.video_url.is_some() {
            query!(
                "INSERT INTO SpeedEvidence
                (solve_id, video_url) 
            VALUES ($1, $2)
            RETURNING *",
                solve.id,
                self.video_url
            )
            .fetch_optional(&state.pool)
            .await?
            .ok_or(AppError::CouldNotInsertSolve)?;
        }

        Ok(UploadSolveResponse {})
    }
}

impl RequestResponse for UploadSolveResponse {
    async fn as_axum_response(self) -> impl IntoResponse {
        "ok"
    }
}

#[derive(serde::Deserialize, Debug, TryFromMultipart)]
pub struct UploadSolveExternal {
    puzzle_id: i32,
    #[serde(deserialize_with = "empty_string_as_none")]
    speed_cs: Option<i32>,
    #[serde(deserialize_with = "on_as_true")]
    blind: bool,
    #[serde(deserialize_with = "empty_string_as_none")]
    memo_cs: Option<i32>,
    #[serde(deserialize_with = "on_as_true")]
    uses_filters: bool,
    #[serde(deserialize_with = "on_as_true")]
    uses_macros: bool,
    #[serde(deserialize_with = "empty_string_as_none")]
    video_url: Option<String>,
    program_version_id: i32,
    #[serde(deserialize_with = "empty_string_as_none")]
    move_count: Option<i32>,
    log_file: Option<String>,
}

pub struct UploadSolveExternalResponse {}

impl RequestBody for UploadSolveExternal {
    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<impl RequestResponse, AppError> {
        let user = user.ok_or(AppError::NotLoggedIn)?;

        let solve = query!(
            "INSERT INTO Solve
                (log_file, user_id, puzzle_id, move_count,
                uses_macros, uses_filters, speed_cs, memo_cs,
                blind, program_version_id) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *",
            self.log_file,
            user.id,
            self.puzzle_id,
            self.move_count,
            self.uses_macros,
            self.uses_filters,
            self.speed_cs,
            if self.blind { self.memo_cs } else { None },
            self.blind,
            self.program_version_id,
        )
        .fetch_optional(&state.pool)
        .await?
        .ok_or(AppError::CouldNotInsertSolve)?;

        if self.video_url.is_some() {
            query!(
                "INSERT INTO SpeedEvidence
                (solve_id, video_url) 
            VALUES ($1, $2)
            RETURNING *",
                solve.id,
                self.video_url
            )
            .fetch_optional(&state.pool)
            .await?
            .ok_or(AppError::CouldNotInsertSolve)?;
        }

        Ok(UploadSolveExternalResponse {})
    }
}

impl RequestResponse for UploadSolveExternalResponse {
    async fn as_axum_response(self) -> impl IntoResponse {
        "ok"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    fn upload_successful(pool: PgPool) -> Result<(), AppError> {
        let state = AppState {
            pool,
            otps: Default::default(),
        };
        let user = state
            .create_user("user@example.com".to_string(), Some("user 1".to_string()))
            .await?;

        let _puzzle_id =
            query!("INSERT INTO Puzzle (hsc_id, name, leaderboard) VALUES ('3x3x3', '3x3x3', NULL) RETURNING id")
                .fetch_one(&state.pool)
                .await?
                .id;

        let program_id =
            query!("INSERT INTO Program (name, abbreviation) VALUES ('Hyperspeedcube', 'HSC') RETURNING id")
                .fetch_one(&state.pool)
                .await?
                .id;

        query!(
            "INSERT INTO ProgramVersion (program_id, version) VALUES ($1, '2.0.0')",
            program_id
        )
        .execute(&state.pool)
        .await?;

        UploadSolveRequest {
            log_file: Some("dummy log file".to_string()),
            video_url: None,
        }
        .request(state, Some(user))
        .await?;

        Ok(())
    }
}
