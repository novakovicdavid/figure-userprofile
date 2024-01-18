use crate::application::profile::service::ProfileServiceError;
use crate::application::transaction::TransactionError;
use crate::application::user::service::UserServiceError;
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

impl ErrorInfo for UserServiceError {
    fn error_message(&self) -> &str {
        match self {
            UserServiceError::UnexpectedError(_) => unreachable!(),
            UserServiceError::UserDomainError(e) => e.error_message(),
            UserServiceError::SecureHasherError(e) => e.error_message(),
            UserServiceError::RepositoryError(e) => e.error_message(),
            UserServiceError::TransactionError(e) => e.error_message(),
            UserServiceError::EmailAlreadyInUse => "email-already-in-use",
            UserServiceError::WrongPassword => "wrong-password",
        }
    }

    fn status_code(&self) -> u16 {
        match self {
            UserServiceError::UnexpectedError(_) => unreachable!(),
            UserServiceError::EmailAlreadyInUse => 409,
            UserServiceError::WrongPassword => 401,
            UserServiceError::UserDomainError(e) => e.status_code(),
            UserServiceError::RepositoryError(e) => e.status_code(),
            UserServiceError::TransactionError(e) => e.status_code(),
            UserServiceError::SecureHasherError(e) => e.status_code(),
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