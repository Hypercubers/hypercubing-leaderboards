use axum::body::Body;
use axum::http::header::SET_COOKIE;
use axum::response::{AppendHeaders, IntoResponse, Redirect, Response};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;

use crate::db::User;
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
    pub turnstile_response: Option<String>,
    pub redirect: Option<String>,
}

pub struct UserRequestOtpResponse {
    redirect: Option<String>,
}

impl RequestBody for UserRequestOtp {
    type Response = UserRequestOtpResponse;

    async fn request(
        self,
        state: AppState,
        _user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        state.verify_turnstile(self.turnstile_response).await?;

        let otp = state.create_otp(self.email.clone());

        #[cfg(debug_assertions)]
        tracing::debug!(otp.code, "otp code");

        crate::email::send_email(
            &self.email,
            "Hypercubing leaderboards sign-in request",
            &format!(
                "Someone tried to sign in to {} using your email address. If this was you, use the following code:\n\n{}\n\nIf you didn't request this, ignore this email.\n\nYou can contact {} for help.",
                *crate::env::DOMAIN_NAME,
                otp.code,
                *crate::env::SUPPORT_EMAIL,
            ),
        )
        .await?;

        Ok(UserRequestOtpResponse {
            redirect: self.redirect,
        })
    }
}

impl IntoResponse for UserRequestOtpResponse {
    fn into_response(self) -> Response<Body> {
        // assume the query parameter is a relative url, which if js/form.js is doing its job will be

        dbg!(&self.redirect);
        // url::Url::parse("/sign-in-otp")
        // format!("sign-in-otp?redirect={}", url_escape self.redirect)

        Redirect::to(
            &format!("/sign-in-otp"),
            // &self
            //     .redirect
            //     .unwrap_or_else(|| self.user.to_public().relative_url()),
        )
        .into_response()
    }
}

pub struct UserRequestToken {
    pub email: String,
    pub otp_code: String,
    pub redirect: Option<String>,
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
        _user: Option<User>,
    ) -> Result<Self::Response, AppError> {
        // Remove expired OTPs
        state.clean_otps();

        let otp = state
            .otps
            .lock()
            .remove(&(self.email, self.otp_code))
            .ok_or(AppError::InvalidOtp)?;
        // OTP is definitely valid because we just cleaned them

        let user = match state.get_user_from_email(&otp.email).await? {
            None => state.create_user(otp.email, None).await?,
            Some(u) => u,
        };

        let token = state.create_token(user.id).await?;
        Ok(TokenReturn {
            user,
            token: token.token,
            redirect: self.redirect,
        })
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

    // #[sqlx::test]
    // fn login_successful(pool: PgPool) -> Result<(), AppError> {
    //     let email = "user@example.com".to_string();
    //     let display_name = "user 1".to_string();
    //     let state = AppState {
    //         pool,
    //         otps: Default::default(),
    //         discord: None,
    //         turnstile: None,
    //     };
    //     println!("email {email}");

    //     UserRequestOtp {
    //         email: email.clone(),
    //         display_name: Some(display_name.clone()),
    //     }
    //     .request(state.clone(), None)
    //     .await?;

    //     // not testing email here, just hack the otp database
    //     let user = state
    //         .get_user_from_email(&email)
    //         .await?
    //         .ok_or(AppError::Other("user does not exist".to_string()))?;
    //     println!("found user: id {}", user.id.0);
    //     let otp_code = state
    //         .otps
    //         .lock()
    //         .get(&user.id)
    //         .ok_or(AppError::Other("otp does not exist".to_string()))?
    //         .code
    //         .clone();
    //     println!("obtained otp: {otp_code}");

    //     let _token_response = UserRequestToken {
    //         email: email.clone(),
    //         otp_code,
    //     }
    //     .request(state.clone(), None)
    //     .await?
    //     .into_response();
    //     println!("token: {:?}", _token_response.headers()[SET_COOKIE]);

    //     Ok(())
    // }

    // #[sqlx::test]
    // fn login_unsuccessful(pool: PgPool) -> Result<(), AppError> {
    //     let email = "user@example.com".to_string();
    //     let display_name = "user 1".to_string();
    //     let state = AppState {
    //         pool,
    //         otps: Default::default(),
    //         discord: None,
    //         turnstile: None,
    //     };

    //     UserRequestOtp {
    //         email: email.clone(),
    //         display_name: Some(display_name.clone()),
    //     }
    //     .request(state.clone(), None)
    //     .await?;

    //     let otp_code = "INVALID OTP".to_string(); // otp codes are always made of digits

    //     let token_response = UserRequestToken {
    //         email: email.clone(),
    //         otp_code,
    //     }
    //     .request(state.clone(), None)
    //     .await;

    //     match token_response {
    //         Ok(_) => Err(AppError::Other(
    //             "login succeeded with incorrect otp".to_string(),
    //         )),
    //         Err(_) => Ok(()),
    //     }
    // }
}
