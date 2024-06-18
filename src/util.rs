use axum::http::StatusCode;

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn to_internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    internal_error(&err.to_string())
}

/// Utility function for mapping any string into a `500 Internal Server Error`
/// response.
pub fn internal_error(msg: &str) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string())
}
