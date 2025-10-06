use std::fmt;

use axum::body::Body;
use axum::extract::multipart::MultipartError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serenity::prelude::SerenityError;

/// Type alias for `AppResult`
pub type AppResult<T = ()> = Result<T, AppError>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum AppError {
    NotFound,

    SqlError(Box<sqlx::Error>),
    EmailError(Box<mail_send::Error>),
    TemplateError(Box<handlebars::RenderError>),
    DoubleTemplateError(Box<handlebars::RenderError>, String),
    IoError(std::io::Error),
    UserDoesNotExist,
    AuthenticationTimeout,
    InvalidOtp,
    InvalidToken,
    DiscordMemberNotFound,
    InvalidDiscordAccount,
    PuzzleVersionDoesNotExist,
    ProgramVersionDoesNotExist,
    CouldNotInsertSolve,
    MultipartError(MultipartError),
    NoLogFile,
    NotLoggedIn,
    InvalidQuery(String),
    NoDiscord,
    DiscordError(Box<SerenityError>),
    NotAuthorized,
    InvalidSolve,
    NoEvidence,
    FailedCaptcha,
    Panic,

    #[allow(dead_code)]
    Other(String),
}

impl AppError {
    pub fn message(&self) -> String {
        match self {
            Self::NotFound => "404 Not Found".to_string(),

            Self::SqlError(err) => format!("Internal SQL error: {err}"),
            Self::EmailError(err) => format!("Email error: {err}"),
            Self::TemplateError(err) => format!("Template error: {err}"),
            Self::DoubleTemplateError(err1, err2) => {
                format!("Double template error: {err1}\n{err2}")
            }
            Self::IoError(err) => format!("IO error: {err}"),
            Self::UserDoesNotExist => "User does not exist".to_string(),
            Self::AuthenticationTimeout => "User took too long to authenticate".to_string(),
            Self::InvalidOtp => "Invalid OTP or device code".to_string(),
            Self::InvalidToken => "Invalid token".to_string(),
            Self::DiscordMemberNotFound => "Discord member not found".to_string(),
            Self::InvalidDiscordAccount => "Invalid Discord account".to_string(),
            Self::PuzzleVersionDoesNotExist => "Puzzle version does not exist".to_string(),
            Self::ProgramVersionDoesNotExist => "Program version does not exist".to_string(),
            Self::CouldNotInsertSolve => "Could not upload solve".to_string(),
            Self::MultipartError(err) => format!("Multipart error: {err}"),
            Self::NoLogFile => "No log file provided".to_string(),
            Self::NotLoggedIn => "Not logged in".to_string(),
            Self::InvalidQuery(err) => format!("Invalid query: {err}"),
            Self::NoDiscord => "Leaderboard is not connected to Discord".to_string(),
            Self::DiscordError(err) => format!("Discord error: {err}"),
            Self::NotAuthorized => "Not authorized".to_string(),
            Self::InvalidSolve => "Invalid solve".to_string(),
            Self::NoEvidence => "No log file or video link provided".to_string(),
            Self::FailedCaptcha => "Failed captcha".to_string(),
            Self::Panic => "Functionality is temporarily disabled".to_string(),

            Self::Other(msg) => msg.to_string(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,

            Self::SqlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EmailError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TemplateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DoubleTemplateError(_, _) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UserDoesNotExist => StatusCode::UNAUTHORIZED,
            Self::AuthenticationTimeout => StatusCode::UNAUTHORIZED,
            Self::InvalidOtp => StatusCode::UNAUTHORIZED,
            Self::InvalidToken => StatusCode::UNAUTHORIZED,
            Self::DiscordMemberNotFound => StatusCode::BAD_REQUEST,
            Self::InvalidDiscordAccount => StatusCode::UNAUTHORIZED,
            Self::PuzzleVersionDoesNotExist => StatusCode::BAD_REQUEST,
            Self::ProgramVersionDoesNotExist => StatusCode::BAD_REQUEST,
            Self::CouldNotInsertSolve => StatusCode::BAD_REQUEST,
            Self::MultipartError(err) => err.status(),
            Self::NoLogFile => StatusCode::BAD_REQUEST,
            Self::NotLoggedIn => StatusCode::UNAUTHORIZED,
            Self::InvalidQuery(_err) => StatusCode::BAD_REQUEST,
            Self::NoDiscord => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DiscordError(_err) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotAuthorized => StatusCode::UNAUTHORIZED,
            Self::InvalidSolve => StatusCode::BAD_REQUEST,
            Self::NoEvidence => StatusCode::BAD_REQUEST,
            Self::FailedCaptcha => StatusCode::BAD_REQUEST,
            Self::Panic => StatusCode::SERVICE_UNAVAILABLE,

            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        (self.status_code(), self.message()).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> AppError {
        AppError::SqlError(Box::new(err))
    }
}

impl From<mail_send::Error> for AppError {
    fn from(err: mail_send::Error) -> AppError {
        AppError::EmailError(Box::new(err))
    }
}

impl From<MultipartError> for AppError {
    fn from(err: MultipartError) -> AppError {
        AppError::MultipartError(err)
    }
}

impl From<SerenityError> for AppError {
    fn from(err: SerenityError) -> AppError {
        AppError::DiscordError(Box::new(err))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for AppError {}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MissingField;
impl fmt::Display for MissingField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unexpected NULL")
    }
}
impl std::error::Error for MissingField {}
impl MissingField {
    pub fn new_sqlx_error(field: &str) -> sqlx::Error {
        sqlx::Error::ColumnDecode {
            index: field.to_string(),
            source: Box::new(Self),
        }
    }
}
