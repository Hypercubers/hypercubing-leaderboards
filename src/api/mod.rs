use axum::body::Body;
use axum::response::{IntoResponse, Redirect, Response};

use crate::db::SolveId;
use crate::traits::Linkable;

pub mod auth;
pub mod autoverify;
pub mod categories;
pub mod edit_user;
pub mod pkce;
pub mod submit_solve;
pub mod verify_solve;

// TODO: give this a better home
#[derive(Debug)]
pub struct UpdateSolveResponse {
    solve_id: SolveId,
}

impl IntoResponse for UpdateSolveResponse {
    fn into_response(self) -> Response<Body> {
        Redirect::to(&self.solve_id.relative_url()).into_response()
    }
}
