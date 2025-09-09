use axum::response::{IntoResponse, Response};
use axum_typed_multipart::TryFromMultipart;

use crate::api::auth::{AuthConfirmAction, AuthContact};
use crate::db::User;
use crate::{AppError, AppState, RequestBody};
