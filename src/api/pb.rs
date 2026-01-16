use axum::response::IntoResponse;
use chrono::{DateTime, Utc};

use crate::{
    db::{
        CategoryQuery, EditAuthorization, FullSolve, ProgramQuery, PuzzleId, SolveId, UserId,
        VariantQuery,
    },
    error::AppError,
    traits::{Linkable, RequestBody},
};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct PbsInCategoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    puzzle_id: Option<PuzzleId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hsc_puzzle_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    target_user: Option<UserId>,

    #[serde(default)]
    average: bool,
    #[serde(default)]
    blind: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    macros: Option<bool>,
    #[serde(default)]
    one_handed: bool,

    require_verified: bool,
}

impl RequestBody for PbsInCategoryRequest {
    type Response = PbsInCategoryResponse;

    async fn request(
        self,
        state: crate::AppState,
        user: Option<crate::db::User>,
    ) -> Result<Self::Response, AppError> {
        let target_user = match self.target_user {
            Some(id) => id,
            None => user.as_ref().ok_or(AppError::NotLoggedIn)?.id,
        };

        let auth = user.as_ref().and_then(|u| u.edit_auth(target_user));

        let require_verified = self.require_verified;

        // Non-moderators may not view unverified solves for other users
        if user.as_ref().map(|u| u.id) != Some(target_user)
            && !require_verified
            && !user.as_ref().is_some_and(|u| u.moderator)
        {
            return Err(AppError::NotAuthorized);
        }

        if self.puzzle_id.is_some() as usize + self.hsc_puzzle_id.is_some() as usize > 1 {
            return Err(AppError::Other(
                "'puzzle_id' and 'hsc_puzzle_id' are mutually exclusive".to_string(),
            ));
        }

        let puzzle_id = if let Some(hsc_puzzle_id) = self.hsc_puzzle_id {
            match state.get_puzzle_with_hsc_id(&hsc_puzzle_id).await? {
                Some(id) => id,
                None => return Ok(PbsInCategoryResponse::default()),
            }
        } else {
            self.puzzle_id.ok_or_else(|| {
                AppError::Other("one of 'puzzle_id' or 'hsc_puzzle_id' is required".to_string())
            })?
        };

        let speed_query = CategoryQuery::Speed {
            average: self.average,
            blind: self.blind,
            filters: self.filters,
            macros: self.macros,
            one_handed: self.one_handed,
            variant: VariantQuery::Default,
            program: ProgramQuery::Default,
        };

        let fmc_query = CategoryQuery::Fmc {
            computer_assisted: false,
        };
        let fmcca_query = CategoryQuery::Fmc {
            computer_assisted: false,
        };

        Ok(PbsInCategoryResponse {
            speed: state
                .pb_in_category(target_user, puzzle_id, &speed_query, require_verified)
                .await?
                .map(|s| PbSolve::from_full_solve(&s, &auth)),
            fmc: state
                .pb_in_category(target_user, puzzle_id, &fmc_query, require_verified)
                .await?
                .map(|s| PbSolve::from_full_solve(&s, &auth)),
            fmcca: state
                .pb_in_category(target_user, puzzle_id, &fmcca_query, require_verified)
                .await?
                .map(|s| PbSolve::from_full_solve(&s, &auth)),
        })
    }
}

#[derive(serde::Serialize, Default)]
pub struct PbsInCategoryResponse {
    /// Speed PB
    speed: Option<PbSolve>,
    /// FMC PB
    fmc: Option<PbSolve>,
    /// Computer-assisted FMC PB
    fmcca: Option<PbSolve>,
}

#[derive(serde::Serialize)]
pub struct PbSolve {
    id: SolveId,
    url: String,
    solve_date: DateTime<Utc>,
    move_count: Option<i32>,
    speed_cs: Option<i32>,
    fmc_verified: Option<bool>,
    speed_verified: Option<bool>,
}

impl PbSolve {
    pub fn from_full_solve(solve: &FullSolve, auth: &Option<EditAuthorization>) -> Self {
        PbSolve {
            id: solve.id,
            url: solve.absolute_url(),
            solve_date: solve.solve_date,
            move_count: solve
                .move_count
                .filter(|_| auth.is_some() || solve.fmc_verified == Some(true)),
            speed_cs: solve
                .speed_cs
                .filter(|_| auth.is_some() || solve.speed_verified == Some(true)),
            fmc_verified: solve.fmc_verified.filter(|&v| v || auth.is_some()),
            speed_verified: solve.speed_verified.filter(|&v| v || auth.is_some()),
        }
    }
}

impl IntoResponse for PbsInCategoryResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
