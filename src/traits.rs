use crate::error::AppError;
use crate::AppState;
use axum::extract::Query;
use axum::extract::State;
use axum::http::header::SET_COOKIE;
use axum::http::Uri;
use axum::response::AppendHeaders;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum_extra::extract::CookieJar;
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};

use crate::db::user::User;

async fn process_jar(
    state: AppState,
    jar: CookieJar,
) -> Result<
    (
        Option<User>,
        AppendHeaders<Vec<(axum::http::HeaderName, String)>>,
    ),
    AppError,
> {
    match jar.get("token") {
        Some(token) => {
            let token = token.value();
            Ok(match state.token_bearer(token).await? {
                Some(user) => (Some(user), AppendHeaders(vec![])),
                None => (
                    None,
                    AppendHeaders(vec![(
                        SET_COOKIE,
                        "token=expired; Expires=Thu, 1 Jan 1970 00:00:00 GMT".to_string(),
                    )]),
                ),
            })
        }
        None => Ok((None, AppendHeaders(vec![]))),
    }
}

/// An object that can be received as a request
pub trait RequestBody {
    type Response;

    async fn request(self, state: AppState, user: Option<User>)
        -> Result<Self::Response, AppError>;

    async fn as_handler_query(
        State(state): State<AppState>,
        uri: Uri,
        jar: CookieJar,
        Query(item): Query<Self>,
    ) -> Result<impl IntoResponse, AppError>
    where
        Self: Sized,
        Self::Response: IntoResponse,
    {
        let (user, headers) = process_jar(state.clone(), jar).await?;
        let response_err = item.request(state, user).await;
        match response_err {
            Err(AppError::NotLoggedIn) => {
                let mut login_redirect =
                    url::Url::parse("https://example.com/sign-in").expect("valid url"); // the url crate cannot handle relative urls
                login_redirect.query_pairs_mut().append_pair(
                    "redirect",
                    &uri.path_and_query()
                        .ok_or_else(|| AppError::Other("no path_and_query".to_string()))?
                        .to_string(),
                ); // it shouldn't panic but i have no idea what could cause this

                return Ok(Redirect::to(&format!(
                    "{}?{}",
                    login_redirect.path(),
                    login_redirect.query().unwrap_or("")
                ))
                .into_response());
            }
            _ => {}
        }

        let response = response_err?;
        Ok((headers, response).into_response())
    }

    /*async fn as_handler_file(
        State(state): State<AppState>,
        jar: CookieJar,
        Query(item): Query<Self>,
        mut multipart: Multipart,
    ) -> Result<impl IntoResponse, AppError>
    where
        Self: Sized,
    {
        let user = if let Some(token) = jar.get("token") {
            let token = token.value();
            state.token_bearer(token).await? // cannot use map() because of this
        } else {
            None
        };

        let log_file = if let Some(field) = multipart.next_field().await? {
            Some(field.text().await?)
        } else {
            None
        };

        item.request(state, user).await
    }*/

    #[allow(dead_code)]
    async fn show_all(request: axum::extract::Request) {
        dbg!(request);
    }

    async fn as_multipart_form_handler(
        State(state): State<AppState>,
        jar: CookieJar,
        TypedMultipart(item): TypedMultipart<Self>,
    ) -> Result<impl IntoResponse, AppError>
    where
        Self: TryFromMultipart,
        Self::Response: IntoResponse,
    {
        let (user, headers) = process_jar(state.clone(), jar).await?;
        let response = item.request(state, user).await?;
        Ok((headers, response))
    }
}
