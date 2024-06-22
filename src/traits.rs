use crate::error::AppError;
use crate::AppState;
use axum::extract::Query;
use axum::extract::State;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};

use crate::db::user::User;

/*trait Buildable {
    async fn build() -> Self;
}*/

pub trait RequestBody {
    async fn request(
        self,
        state: AppState,
        user: Option<User>,
    ) -> Result<impl RequestResponse, AppError>;

    async fn as_handler_query(
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
        let response = item.request(state, user).await?;
        Ok(response.as_axum_response().await)
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
        let response = item.request(state, user).await?;
        Ok(response.as_axum_response().await)
    }
}

pub trait RequestResponse
where
    Self: Send,
{
    fn as_axum_response(self) -> impl std::future::Future<Output = impl IntoResponse> + Send;
}
