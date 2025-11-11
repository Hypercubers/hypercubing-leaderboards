use axum::extract::{Json, Query, State};
use axum::http::Uri;
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::CookieJar;
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};

use crate::db::User;
use crate::{AppError, AppResult, AppState};

/// Object that can be linked in Markdown.
pub trait Linkable {
    /// Returns the relative URL. Example: `/solve?id=3`
    fn relative_url(&self) -> String;

    /// Returns the absolute URL. Example: `https://lb.hypercubing.xyz/solve?id=3`
    fn absolute_url(&self) -> String {
        crate::env::DOMAIN_NAME.clone() + &self.relative_url()
    }

    /// Returns Markdown text for the object.
    fn md_text(&self) -> String;

    /// Returns a Markdown link to the object.
    fn md_link(&self, bold: bool) -> String {
        let f = if bold { "**" } else { "" };
        format!(
            "[{f}{}{f}](<{}>)",
            &self.md_text().replace(['[', ']'], ""), // discord just doesn't let us do proper escaping
            self.absolute_url(),
        )
    }
}

/// Extension trait for Discord command context.
pub trait PoiseCtxExt {
    /// Returns the [`User`] who issued the command, or `None` if the command
    /// issuer does not have an account.
    async fn opt_author_user(&self) -> Result<Option<User>, AppError>;
    /// Returns the [`User`] who issued the command, or an error if the command
    /// issuer does not have an account.
    async fn author_user(&self) -> Result<User, AppError>;
}
impl PoiseCtxExt for PoiseCtx<'_> {
    async fn opt_author_user(&self) -> Result<Option<User>, AppError> {
        Ok(self
            .data()
            .get_opt_user_from_discord_id(self.author().id.into())
            .await?)
    }
    async fn author_user(&self) -> Result<User, AppError> {
        self.opt_author_user().await?.ok_or(AppError::NotLoggedIn)
    }
}

/// Discord command context.
pub type PoiseCtx<'a> = poise::Context<'a, AppState, AppError>;

/// Object that can be received as a request.
pub trait RequestBody {
    type Response;

    async fn request(self, state: AppState, user: Option<User>)
    -> Result<Self::Response, AppError>;

    async fn preprocess_jar(_state: &AppState, _jar: &CookieJar) -> AppResult {
        Ok(())
    }

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
        Self::preprocess_jar(&state, &jar).await?;
        let (user, headers) = crate::cookies::process_cookies(&state, &jar).await?;
        let response_err = item.request(state, user).await;
        match response_err {
            Err(AppError::NotLoggedIn) => {
                let mut login_redirect =
                    url::Url::parse("https://example.com/sign-in").expect("valid url"); // the url crate cannot handle relative urls
                if let Some(path_and_query) = uri.path_and_query() {
                    login_redirect
                        .query_pairs_mut()
                        .append_pair("redirect", &path_and_query.to_string());
                }

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
        let (user, headers) = crate::cookies::process_cookies(&state, &jar).await?;
        let response = item.request(state, user).await?;
        Ok((headers, response))
    }

    async fn as_json_handler(
        State(state): State<AppState>,
        jar: CookieJar,
        Json(item): Json<Self>,
    ) -> Result<impl IntoResponse, AppError>
    where
        Self: for<'de> serde::Deserialize<'de>,
        Self::Response: IntoResponse,
    {
        let (user, headers) = crate::cookies::process_cookies(&state, &jar).await?;
        let response = item.request(state, user).await?;
        Ok((headers, response))
    }

    /// Performs the request and sends the response as a reply in Discord.
    async fn request_via_discord(self, ctx: &PoiseCtx<'_>) -> AppResult<Self::Response>
    where
        Self: Sized,
    {
        self.request(ctx.data().clone(), ctx.opt_author_user().await?)
            .await
    }
}
