use axum::body::{Body, Bytes};
use axum::extract::Multipart;
use axum::response::{IntoResponse, Redirect, Response};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipartError};
use chrono::{NaiveDate, NaiveTime};
use futures::FutureExt;

use crate::db::{SolveDbFields, SolveId, User, UserId};
use crate::traits::{Linkable, RequestBody};
use crate::{AppError, AppState};

#[derive(Debug, TryFromMultipart)]
pub struct SolveData {
    pub solve_id: Option<i32>,

    // Event
    pub puzzle_id: i32,
    pub variant_id: Option<i32>,
    pub program_id: i32,

    // Metadata
    pub solver_id: Option<i32>,
    pub solve_date: NaiveDate,
    pub solver_notes: Option<String>,
    pub moderator_notes: Option<String>,

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
    pub replace_log_file: Option<bool>,
    pub log_file: Option<FieldData<Bytes>>,
}
impl SolveData {
    fn total_speed_cs(&self) -> Option<i32> {
        Self::total_cs([self.solve_h, self.solve_m, self.solve_s, self.solve_cs])
    }

    fn total_memo_cs(&self) -> Option<i32> {
        Self::total_cs([self.memo_h, self.memo_m, self.memo_s, self.memo_cs])
    }

    fn total_cs([h, m, s, cs]: [Option<i32>; 4]) -> Option<i32> {
        let mut total_cs = h.unwrap_or(0);
        total_cs *= 60;
        total_cs += m.unwrap_or(0);
        total_cs *= 60;
        total_cs += s.unwrap_or(0);
        total_cs *= 100;
        total_cs += cs.unwrap_or(0);
        (total_cs != 0).then_some(total_cs)
    }

    pub fn into_raw(self, default_solver: UserId) -> SolveDbFields {
        let speed_cs = self.total_speed_cs();
        let memo_cs = self.total_memo_cs();

        let Self {
            solver_id,
            solve_id: _,
            puzzle_id,
            variant_id,
            program_id,
            solve_date,
            solver_notes,
            moderator_notes,
            solve_h: _,
            solve_m: _,
            solve_s: _,
            solve_cs: _,
            uses_filters,
            uses_macros,
            average,
            one_handed,
            blind,
            memo_h: _,
            memo_m: _,
            memo_s: _,
            memo_cs: _,
            video_url,
            move_count,
            computer_assisted,
            replace_log_file,
            log_file,
        } = self;

        dbg!(&solver_notes);
        dbg!(&moderator_notes);

        let is_speed = speed_cs.is_some();
        let is_fmc = move_count.is_some();

        let log_file = (replace_log_file != Some(false)).then(|| {
            log_file.map(|data| {
                let get_default_name = || "unknown.txt".to_string();
                let file_name = data.metadata.file_name.unwrap_or_else(get_default_name);
                (file_name, data.contents.into())
            })
        });

        SolveDbFields {
            puzzle_id,
            variant_id,
            program_id,
            solver_id: solver_id.unwrap_or(default_solver.0),
            solve_date: solve_date.and_time(NaiveTime::default()).and_utc(),
            solver_notes: solver_notes.unwrap_or_default().replace('\r', ""),
            moderator_notes: Some(moderator_notes.unwrap_or_default().replace('\r', "")),
            average,
            blind: is_speed && blind,
            filters: is_speed && uses_filters,
            macros: is_speed && uses_macros,
            one_handed: is_speed && one_handed,
            computer_assisted: is_fmc && computer_assisted,
            move_count,
            speed_cs,
            memo_cs: memo_cs.filter(|_| is_speed && blind),
            log_file,
            video_url,
        }
    }
}

pub struct ManualSubmitSolveRequest(pub SolveData);
impl_try_from_multipart_wrapper!(ManualSubmitSolveRequest(SolveData));
impl RequestBody for ManualSubmitSolveRequest {
    type Response = UpdateSolveResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let user = user.ok_or(AppError::NotLoggedIn)?;

        let solve_data = self.0;

        if solve_data.video_url.is_none() && solve_data.log_file.is_none() {
            return Err(AppError::NoEvidence);
        }

        let solve_id = state
            .add_solve_external(&user, solve_data.into_raw(user.id))
            .await?;

        Ok(UpdateSolveResponse { solve_id })
    }
}

pub struct UpdateSolveRequest(pub SolveData);
impl_try_from_multipart_wrapper!(UpdateSolveRequest(SolveData));
impl RequestBody for UpdateSolveRequest {
    type Response = UpdateSolveResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let editor = user.ok_or(AppError::NotLoggedIn)?;
        let solve_data = self.0;
        let solve_id = SolveId(solve_data.solve_id.ok_or(AppError::InvalidSolve)?);

        let solver_id = solve_data
            .solver_id
            .ok_or_else(|| AppError::Other("Missing solver ID".to_string()))?;

        state
            .update_solve(solve_id, solve_data.into_raw(UserId(solver_id)), &editor)
            .await?;

        Ok(UpdateSolveResponse { solve_id })
    }
}

pub struct UpdateSolveResponse {
    solve_id: SolveId,
}

impl IntoResponse for UpdateSolveResponse {
    fn into_response(self) -> Response<Body> {
        Redirect::to(&self.solve_id.relative_url()).into_response()
    }
}

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
