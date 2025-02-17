use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;

use crate::{error::AppError, AppState};

/// Fallback route handler that returns a 404 error.
#[axum::debug_handler]
pub async fn handler_query(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Response, AppError> {
    let (user, headers) = crate::cookies::process_cookies(&state, &jar).await?;
    Ok((
        axum::http::StatusCode::NOT_FOUND,
        headers,
        crate::render_html_template("404.html", &user, serde_json::json!({})),
    )
        .into_response())
}
