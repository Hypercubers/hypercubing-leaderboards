use axum::response::AppendHeaders;
use axum_extra::extract::CookieJar;

use crate::{
    api::auth::{APPEND_EXPIRED_TOKEN, APPEND_NO_TOKEN},
    db::User,
    error::AppError,
    AppState,
};

pub async fn process_cookies(
    state: &AppState,
    jar: &CookieJar,
) -> Result<
    (
        Option<User>,
        AppendHeaders<Option<(axum::http::HeaderName, &'static str)>>,
    ),
    AppError,
> {
    match jar.get("token") {
        Some(token) => {
            let token = token.value();
            Ok(match state.token_bearer(token).await? {
                Some(user) => (Some(user), APPEND_NO_TOKEN),
                None => (None, APPEND_EXPIRED_TOKEN),
            })
        }
        None => Ok((None, APPEND_NO_TOKEN)),
    }
}
