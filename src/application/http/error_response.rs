use axum::Json;
use axum_core::response::{IntoResponse, Response};
use figure_lib::error_handling::IntoHttpStatusCode;
use http::StatusCode;
use serde::Serialize;
use tracing::{error, Instrument, Span};

use crate::application::ApplicationError;
use crate::application::connectors::auth_connector::AuthConnectorError;
use crate::application::errors::{RepositoryError, RouteError};
use crate::application::profile::service::ProfileServiceError;
use crate::application::transaction::TransactionError;
use crate::application::user_profile::service::UserProfileServiceError;
use crate::domain::profile::ProfileDomainError;
use crate::domain::user::UserDomainError;
use crate::infrastructure::secure_hasher::SecureHasherError;

#[derive(Serialize)]
pub struct ErrorResponse<'a> {
    error: &'a str,
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        let error_str = self.to_string();

        let status_code = match self {
            ApplicationError::UnexpectedError(error) => {
                tokio::task::spawn(async move {
                    error!("Internal server error: {}\n{}", error, error.backtrace());
                }.instrument(Span::current()));

                500
            }

            ApplicationError::UserProfileServiceError(e) => e.status_code(),
            ApplicationError::ProfileServiceError(e) => e.status_code(),
            ApplicationError::RouteError(e) => e.status_code(),
        };

        (
            StatusCode::from_u16(status_code).unwrap(),
            Json(ErrorResponse {
                error: &error_str
            })
        ).into_response()
    }
}

impl IntoHttpStatusCode for ApplicationError {
    fn status_code(&self) -> u16 {
        match self {
            ApplicationError::UnexpectedError(_) => 500,
            ApplicationError::UserProfileServiceError(e) => e.status_code(),
            ApplicationError::ProfileServiceError(e) => e.status_code(),
            ApplicationError::RouteError(e) => e.status_code(),
        }
    }
}

impl IntoHttpStatusCode for UserDomainError {
    fn status_code(&self) -> u16 {
        match self {
            UserDomainError::InvalidEmail => 400,
            UserDomainError::PasswordTooShort => 400,
            UserDomainError::PasswordTooLong => 400
        }
    }
}

impl IntoHttpStatusCode for ProfileDomainError {
    fn status_code(&self) -> u16 {
        match self {
            ProfileDomainError::InvalidUsername => 400
        }
    }
}

impl IntoHttpStatusCode for UserProfileServiceError {
    fn status_code(&self) -> u16 {
        match self {
            UserProfileServiceError::UnexpectedError(_) => unreachable!(),
            UserProfileServiceError::EmailAlreadyInUse => 409,
            UserProfileServiceError::UserDomainError(e) => e.status_code(),
            UserProfileServiceError::ProfileDomainError(e) => e.status_code(),
            UserProfileServiceError::RepositoryError(e) => e.status_code(),
            UserProfileServiceError::TransactionError(e) => e.status_code(),
            UserProfileServiceError::SecureHasherError(e) => e.status_code(),
            UserProfileServiceError::AuthConnectorError(e) => e.status_code(),
        }
    }
}

impl IntoHttpStatusCode for ProfileServiceError {
    fn status_code(&self) -> u16 {
        match self {
            ProfileServiceError::UnexpectedError(_) => unreachable!(),
            ProfileServiceError::RepositoryError(e) => e.status_code()
        }
    }
}

impl IntoHttpStatusCode for SecureHasherError {
    fn status_code(&self) -> u16 {
        match self {
            SecureHasherError::UnexpectedError(_) => unreachable!(),
            SecureHasherError::WrongPassword => 401
        }
    }
}

impl IntoHttpStatusCode for TransactionError {
    fn status_code(&self) -> u16 {
        match self {
            TransactionError::UnexpectedError(_) => unreachable!()
        }
    }
}

impl IntoHttpStatusCode for AuthConnectorError {
    fn status_code(&self) -> u16 {
        match self {
            AuthConnectorError::UnexpectedError(_) => unreachable!(),
        }
    }
}

impl IntoHttpStatusCode for RouteError {
    fn status_code(&self) -> u16 {
        match self {
            RouteError::InvalidMultipart => 400
        }
    }
}

impl IntoHttpStatusCode for RepositoryError {
    fn status_code(&self) -> u16 {
        match self {
            RepositoryError::UnexpectedError(_) => unreachable!(),
            RepositoryError::ResourceNotFound => 404,
            RepositoryError::ConstraintConflict => 409,
        }
    }
}