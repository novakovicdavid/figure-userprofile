use std::fmt::{Display, Formatter};
use crate::application::connectors::auth_connector::AuthConnectorError;
use crate::application::profile::service::ProfileServiceError;
use crate::application::transaction::TransactionError;
use crate::application::user_profile::service::UserProfileServiceError;
use crate::domain::profile::ProfileDomainError;
use crate::domain::user::UserDomainError;
use crate::infrastructure::secure_hasher::SecureHasherError;

pub trait ErrorInfo {
    fn error_message(&self) -> &str;
    fn status_code(&self) -> u16;
}

impl ErrorInfo for UserDomainError {
    fn error_message(&self) -> &str {
        match self {
            UserDomainError::InvalidEmail => "invalid-email",
            UserDomainError::PasswordTooShort => "password-too-short",
            UserDomainError::PasswordTooLong => "password-too-long",
        }
    }

    fn status_code(&self) -> u16 {
        match self {
            UserDomainError::InvalidEmail => 400,
            UserDomainError::PasswordTooShort => 400,
            UserDomainError::PasswordTooLong => 400
        }
    }
}

impl ErrorInfo for ProfileDomainError {
    fn error_message(&self) -> &str {
        match self {
            ProfileDomainError::InvalidUsername => "invalid-username",
        }
    }

    fn status_code(&self) -> u16 {
        match self {
            ProfileDomainError::InvalidUsername => 400
        }
    }
}

impl ErrorInfo for UserProfileServiceError {
    fn error_message(&self) -> &str {
        match self {
            UserProfileServiceError::UnexpectedError(_) => unreachable!(),
            UserProfileServiceError::UserDomainError(e) => e.error_message(),
            UserProfileServiceError::ProfileDomainError(e) => e.error_message(),
            UserProfileServiceError::SecureHasherError(e) => e.error_message(),
            UserProfileServiceError::RepositoryError(e) => e.error_message(),
            UserProfileServiceError::TransactionError(e) => e.error_message(),
            UserProfileServiceError::AuthConnectorError(e) => e.error_message(),
            UserProfileServiceError::EmailAlreadyInUse => "email-already-in-use",
            UserProfileServiceError::WrongPassword => "wrong-password",
        }
    }

    fn status_code(&self) -> u16 {
        match self {
            UserProfileServiceError::UnexpectedError(_) => unreachable!(),
            UserProfileServiceError::EmailAlreadyInUse => 409,
            UserProfileServiceError::WrongPassword => 401,
            UserProfileServiceError::UserDomainError(e) => e.status_code(),
            UserProfileServiceError::ProfileDomainError(e) => e.status_code(),
            UserProfileServiceError::RepositoryError(e) => e.status_code(),
            UserProfileServiceError::TransactionError(e) => e.status_code(),
            UserProfileServiceError::SecureHasherError(e) => e.status_code(),
            UserProfileServiceError::AuthConnectorError(e) => e.status_code(),
        }
    }
}

impl ErrorInfo for ProfileServiceError {
    fn error_message(&self) -> &str {
        match self {
            ProfileServiceError::UnexpectedError(_) => unreachable!(),
            ProfileServiceError::RepositoryError(e) => e.error_message(),
        }
    }

    fn status_code(&self) -> u16 {
        match self {
            ProfileServiceError::UnexpectedError(_) => unreachable!(),
            ProfileServiceError::RepositoryError(e) => e.status_code()
        }
    }
}

impl ErrorInfo for SecureHasherError {
    fn error_message(&self) -> &str {
        match self {
            SecureHasherError::UnexpectedError(_) => unreachable!(),
            SecureHasherError::WrongPassword => "wrong-password"
        }
    }

    fn status_code(&self) -> u16 {
        match self {
            SecureHasherError::UnexpectedError(_) => unreachable!(),
            SecureHasherError::WrongPassword => 401
        }
    }
}

impl ErrorInfo for TransactionError {
    fn error_message(&self) -> &str {
        match self {
            TransactionError::UnexpectedError(_) => unreachable!()
        }
    }

    fn status_code(&self) -> u16 {
        match self {
            TransactionError::UnexpectedError(_) => unreachable!()
        }
    }
}

impl ErrorInfo for AuthConnectorError {
    fn error_message(&self) -> &str {
        match self {
            AuthConnectorError::UnexpectedError(_) => unreachable!(),
        }
    }

    fn status_code(&self) -> u16 {
        match self {
            AuthConnectorError::UnexpectedError(_) => unreachable!(),
        }
    }
}