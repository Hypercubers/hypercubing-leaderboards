use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::CookieJar;

use crate::{AppError, AppState};

/// Fallback route handler that returns a 404 error.
pub async fn handler_query(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Response, AppError> {
    let (user, headers) = crate::cookies::process_cookies(&state, &jar).await?;
    let html = crate::render_html_template("404.html", &user, serde_json::json!({}));
    Ok((StatusCode::NOT_FOUND, headers, html).into_response())
}
