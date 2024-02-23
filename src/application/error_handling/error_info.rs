use crate::application::connectors::auth_connector::AuthConnectorError;
use crate::application::profile::service::ProfileServiceError;
use crate::application::transaction::TransactionError;
use crate::application::user_profile::service::UserProfileServiceError;
use crate::domain::profile::ProfileDomainError;
use crate::domain::user::UserDomainError;
use crate::infrastructure::secure_hasher::SecureHasherError;

pub trait ErrorInfo {
    fn status_code(&self) -> u16;
}

impl ErrorInfo for UserDomainError {
    fn status_code(&self) -> u16 {
        match self {
            UserDomainError::InvalidEmail => 400,
            UserDomainError::PasswordTooShort => 400,
            UserDomainError::PasswordTooLong => 400
        }
    }
}

impl ErrorInfo for ProfileDomainError {
    fn status_code(&self) -> u16 {
        match self {
            ProfileDomainError::InvalidUsername => 400
        }
    }
}

impl ErrorInfo for UserProfileServiceError {
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

impl ErrorInfo for ProfileServiceError {
    fn status_code(&self) -> u16 {
        match self {
            ProfileServiceError::UnexpectedError(_) => unreachable!(),
            ProfileServiceError::RepositoryError(e) => e.status_code()
        }
    }
}

impl ErrorInfo for SecureHasherError {
    fn status_code(&self) -> u16 {
        match self {
            SecureHasherError::UnexpectedError(_) => unreachable!(),
            SecureHasherError::WrongPassword => 401
        }
    }
}

impl ErrorInfo for TransactionError {
    fn status_code(&self) -> u16 {
        match self {
            TransactionError::UnexpectedError(_) => unreachable!()
        }
    }
}

impl ErrorInfo for AuthConnectorError {
    fn status_code(&self) -> u16 {
        match self {
            AuthConnectorError::UnexpectedError(_) => unreachable!(),
        }
    }
}