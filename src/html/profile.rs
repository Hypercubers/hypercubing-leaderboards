use axum::response::{IntoResponse, Redirect, Response};

use crate::api::profile::*;
use crate::HtmlResponse;

impl HtmlResponse for UpdateUserDiscordIdResponse {
    async fn into_html(self) -> Response {
        Redirect::to(&self.target_user_id.relative_url()).into_response()
    }
}
impl HtmlResponse for UpdateUserEmailResponse {
    async fn into_html(self) -> Response {
        Redirect::to(&self.target_user_id.relative_url()).into_response()
    }
}
impl HtmlResponse for UpdateUserNameResponse {
    async fn into_html(self) -> Response {
        Redirect::to(&self.target_user_id.relative_url()).into_response()
    }
}
