use axum::body::Body;
use axum::response::{Html, IntoResponse, Response};

use crate::db::program::ProgramVersion;
use crate::db::puzzle::Puzzle;
pub use crate::db::solve::FullSolve;
use crate::db::solve::SolveId;
use crate::db::user::User;
use crate::error::AppError;
use crate::traits::RequestBody;
use crate::{AppState, HBS};

#[derive(serde::Deserialize)]
pub struct SignedOutPage {}

pub struct SignedOutPageResponse {
    user: Option<User>,
}

impl RequestBody for SignedOutPage {
    type Response = SignedOutPageResponse;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        Ok(SignedOutPageResponse { user })
    }
}

impl IntoResponse for SignedOutPageResponse {
    fn into_response(self) -> Response<Body> {
        Html(
            HBS
                .render(
                    "signed-out.html",
                    &serde_json::json!({
                        "active_user": self.user.map(|u|u.to_public().to_header_json()).unwrap_or_default()
                    }),
                )
                .expect("render error"),
        )
        .into_response()
    }
}
