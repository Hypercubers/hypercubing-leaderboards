use axum::response::Html;
use axum::response::IntoResponse;

pub async fn upload_external() -> impl IntoResponse {
    Html(include_str!("../../html/upload-external.html"))
}

pub async fn sign_in() -> impl IntoResponse {
    Html(include_str!("../../html/sign-in.html"))
}
