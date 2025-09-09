use axum::response::AppendHeaders;
use axum_extra::extract::CookieJar;

use crate::api::auth::{APPEND_EXPIRED_TOKEN, APPEND_NO_TOKEN};
use crate::db::token::TokenStatus;
use crate::db::User;
use crate::{AppError, AppState};

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
    let token = jar.get("token").map(|cookie| cookie.value());
    let token_status = state.token_status(token).await?;
    let cookie_header = match &token_status {
        TokenStatus::None | TokenStatus::Valid(_) => APPEND_NO_TOKEN,
        TokenStatus::Expired | TokenStatus::Unknown => APPEND_EXPIRED_TOKEN,
    };
    let user = match token_status {
        TokenStatus::Valid(user) => Some(user),
        _ => None,
    };
    Ok((user, cookie_header))
}
