use crate::error::AppError;
use crate::AppState;
use axum::extract::Query;
use axum::extract::{Multipart, State};
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;

use crate::db::User;

/*trait Buildable {
    async fn build() -> Self;
}*/

pub trait RequestBody {
    async fn request(
        self,
        state: AppState,
        user: Option<User>,
        log_file: Option<String>,
    ) -> Result<impl IntoResponse, AppError>;

    async fn as_handler(
        State(state): State<AppState>,
        jar: CookieJar,
        Query(item): Query<Self>,
    ) -> Result<impl IntoResponse, AppError>
    where
        Self: Sized,
    {
        let user = match jar.get("token") {
            Some(token) => {
                let token = token.value();
                Some(
                    state
                        .token_bearer(token)
                        .await?
                        .ok_or(AppError::InvalidToken)?,
                ) // cannot use map() because of this ?
            }
            None => None,
        };
        item.request(state, user, None).await
    }

    async fn as_handler_file(
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

        item.request(state, user, log_file).await
    }
}
