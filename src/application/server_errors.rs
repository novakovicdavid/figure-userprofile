use serde::{Serialize};
use std::fmt::{Display, Formatter};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use tracing::{error, Instrument, Span};
use crate::domain::profile::errors::ProfileError;
use crate::domain::user::errors::UserError;

#[derive(Debug)]
pub enum ServerError {
    UserError(UserError),
    ProfileError(ProfileError),
    EmailAlreadyInUse,
    UsernameAlreadyTaken,
    WrongPassword,
    ResourceNotFound,
    // No session cookie received
    NoSessionReceived,
    InvalidImage,
    MissingFieldInForm,
    InvalidMultipart,
    ImageDimensionsTooLarge,
    InternalError(anyhow::Error),
}

impl ServerError {
    pub fn to_str(&self) -> &str {
        match self {
            ServerError::UserError(user_error) => match user_error {
                UserError::InvalidEmail => "invalid-email",
                UserError::PasswordTooShort => "password-too-short",
                UserError::PasswordTooLong => "password-too-long",
            },
            ServerError::ProfileError(profile_error) => match profile_error {
                ProfileError::InvalidUsername => "invalid-username"
            },
            ServerError::EmailAlreadyInUse => "email-already-in-use",
            ServerError::UsernameAlreadyTaken => "username-already-taken",
            ServerError::WrongPassword => "wrong-password",
            ServerError::ResourceNotFound => "resource-not-found",
            ServerError::NoSessionReceived => "no-session-received",
            ServerError::InvalidImage => "invalid-image",
            ServerError::MissingFieldInForm => "missing-field-in-form",
            ServerError::InvalidMultipart => "invalid-multipart",
            ServerError::ImageDimensionsTooLarge => "image-dimensions-too-large",
            ServerError::InternalError(_) => "internal-server-error",

        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse<'a> {
    error: &'a str,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        let error_str = self.to_string();

        let status_code = match self {
            ServerError::UserError(user_error) => match user_error {
                UserError::InvalidEmail => StatusCode::BAD_REQUEST,
                UserError::PasswordTooShort => StatusCode::BAD_REQUEST,
                UserError::PasswordTooLong => StatusCode::BAD_REQUEST,
            },
            ServerError::ProfileError(profile_error) => match profile_error {
                ProfileError::InvalidUsername => StatusCode::BAD_REQUEST
            },
            ServerError::EmailAlreadyInUse => StatusCode::BAD_REQUEST,
            ServerError::UsernameAlreadyTaken => StatusCode::BAD_REQUEST,
            ServerError::WrongPassword => StatusCode::BAD_REQUEST,
            ServerError::ResourceNotFound => StatusCode::NOT_FOUND,
            ServerError::NoSessionReceived => StatusCode::BAD_REQUEST,
            ServerError::InvalidImage => StatusCode::BAD_REQUEST,
            ServerError::MissingFieldInForm => StatusCode::BAD_REQUEST,
            ServerError::InvalidMultipart => StatusCode::BAD_REQUEST,
            ServerError::ImageDimensionsTooLarge => StatusCode::BAD_REQUEST,
            ServerError::InternalError(error) => {
                tokio::task::spawn(async move {
                    error!("Internal server error: {}\n{}", error, error.backtrace());
                }.instrument(Span::current()));

                StatusCode::INTERNAL_SERVER_ERROR
            },
        };

        (
            status_code,
            Json(ErrorResponse {
                error: &error_str
            })
        ).into_response()
    }
}

impl std::error::Error for ServerError {}

impl PartialEq for ServerError {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl From<UserError> for ServerError {
    fn from(value: UserError) -> Self {
        ServerError::UserError(value)
    }
}

impl From<ProfileError> for ServerError {
    fn from(value: ProfileError) -> Self {
        ServerError::ProfileError(value)
    }
}