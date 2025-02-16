use axum::body::Body;
use axum::extract::State;
use axum::http::header::SET_COOKIE;
use axum::http::StatusCode;
use axum::response::{AppendHeaders, IntoResponse, Redirect, Response};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;

use crate::db::user::User;
use crate::error::AppError;
use crate::traits::{Linkable, RequestBody};
use crate::AppState;

const EXPIRED_TOKEN: &str = "token=expired; Expires=Thu, 1 Jan 1970 00:00:00 GMT";
pub const APPEND_EXPIRED_TOKEN: AppendHeaders<Option<(axum::http::header::HeaderName, &str)>> =
    AppendHeaders(Some((SET_COOKIE, EXPIRED_TOKEN)));
pub const APPEND_NO_TOKEN: AppendHeaders<Option<(axum::http::header::HeaderName, &str)>> =
    AppendHeaders(None);

pub struct UserRequestOtp {
    pub email: String,
    pub display_name: Option<String>,
}

pub struct UserRequestOtpResponse {
    code: StatusCode,
}

impl RequestBody for UserRequestOtp {
    type Response = UserRequestOtpResponse;

    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let db_user = state.get_user_from_email(&self.email).await?;
        let user;
        let created;
        match db_user {
            None => {
                created = true;
                user = state
                    .create_user(self.email.clone(), self.display_name.clone())
                    .await?;
            }
            Some(user_) => {
                created = false;
                user = user_;
            }
        }

        let otp = state.create_otp(user.id);

        #[cfg(debug_assertions)]
        tracing::debug!(otp.code, "otp code");

        // TODO: send an email here

        if created {
            Ok(UserRequestOtpResponse {
                code: StatusCode::CREATED,
            })
        } else {
            Ok(UserRequestOtpResponse {
                code: StatusCode::ACCEPTED,
            })
        }
    }
}

impl IntoResponse for UserRequestOtpResponse {
    fn into_response(self) -> Response<Body> {
        self.code.into_response()
    }
}

pub struct UserRequestToken {
    pub email: String,
    pub otp_code: String,
}

pub struct TokenReturn {
    pub user: User,
    pub token: String,
    pub redirect: Option<String>,
}

impl RequestBody for UserRequestToken {
    type Response = TokenReturn;

    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        let user = state
            .get_user_from_email(&self.email)
            .await?
            .ok_or(AppError::UserDoesNotExist)?;

        let maybe_otp = state.otps.lock().get(&user.id).cloned(); // do not let it lock forever!
        state.clean_otps(); // remove all the expired ones
        if let Some(otp) = maybe_otp {
            if dbg!(otp.is_valid()) && self.otp_code == otp.code {
                // is_valid should be true if the state was cleaned
                // valid otp, remove it since it has been used
                state.otps.lock().remove(&user.id);
                let token = state.create_token(user.id).await?;
                return Ok(TokenReturn {
                    user,
                    token: token.token,
                    redirect: None,
                });
            }
        }
        Err(AppError::InvalidOtp)
    }
}

impl IntoResponse for TokenReturn {
    fn into_response(self) -> Response<Body> {
        let cookie = Cookie::build(("token", self.token))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict);
        let jar = CookieJar::new().add(cookie);

        // assume the query parameter is a relative url, which if js/form.js is doing its job will be
        (
            jar,
            Redirect::to(
                &self
                    .redirect
                    .unwrap_or_else(|| self.user.to_public().relative_url()),
            ),
        )
            .into_response()
    }
}

pub async fn invalidate_current_token(
    state: &AppState,
    jar: &CookieJar,
) -> Result<impl IntoResponse, AppError> {
    // it can't be RequestBody because it needs the token

    let Some(token) = jar.get("token") else {
        return Ok((APPEND_NO_TOKEN, "not signed in"));
    };
    let token = token.value();
    state.remove_token(token).await?;
    Ok((APPEND_EXPIRED_TOKEN, "ok"))
}

#[cfg(test)]
mod tests {
    use axum::http::header::SET_COOKIE;
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test]
    fn login_successful(pool: PgPool) -> Result<(), AppError> {
        let email = "user@example.com".to_string();
        let display_name = "user 1".to_string();
        let state = AppState {
            pool,
            otps: Default::default(),
            discord: None,
        };
        println!("email {}", email);

        UserRequestOtp {
            email: email.clone(),
            display_name: Some(display_name.clone()),
        }
        .request(state.clone(), None)
        .await?;

        // not testing email here, just hack the otp database
        let user = state
            .get_user_from_email(&email)
            .await?
            .ok_or(AppError::Other("user does not exist".to_string()))?;
        println!("found user: id {}", user.id.0);
        let otp_code = state
            .otps
            .lock()
            .get(&user.id)
            .ok_or(AppError::Other("otp does not exist".to_string()))?
            .code
            .clone();
        println!("obtained otp: {}", otp_code);

        let _token_response = UserRequestToken {
            email: email.clone(),
            otp_code,
        }
        .request(state.clone(), None)
        .await?
        .into_response();
        println!("token: {:?}", _token_response.headers()[SET_COOKIE]);

        Ok(())
    }

    #[sqlx::test]
    fn login_unsuccessful(pool: PgPool) -> Result<(), AppError> {
        let email = "user@example.com".to_string();
        let display_name = "user 1".to_string();
        let state = AppState {
            pool,
            otps: Default::default(),
            discord: None,
        };

        UserRequestOtp {
            email: email.clone(),
            display_name: Some(display_name.clone()),
        }
        .request(state.clone(), None)
        .await?;

        let otp_code = "INVALID OTP".to_string(); // otp codes are always made of digits

        let token_response = UserRequestToken {
            email: email.clone(),
            otp_code,
        }
        .request(state.clone(), None)
        .await;

        match token_response {
            Ok(_) => Err(AppError::Other(
                "login succeeded with incorrect otp".to_string(),
            )),
            Err(_) => Ok(()),
        }
    }
}
