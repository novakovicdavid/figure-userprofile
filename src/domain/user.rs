pub use user::User;
pub use user::UserDomainError;

pub mod user {
    use std::time::SystemTime;

    use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
    use argon2::password_hash::{Error, SaltString};
    use error_conversion_macro::ErrorEnum;
    use figure_lib::queue::events::user::PasswordChangedEvent;
    use lazy_static::lazy_static;
    use rand_core::OsRng;
    use regex::Regex;
    use thiserror::Error;
    use unicode_segmentation::UnicodeSegmentation;
    use uuid::Uuid;

    use crate::domain::Profile;
    use crate::domain::profile::ProfileDomainError;
    use crate::infrastructure::secure_hasher::ARGON2_HASHER;

    #[derive(Debug, Clone, PartialEq)]
    pub struct User {
        pub id: String,
        pub email: String,
        pub password: String,
        pub role: String,
    }

    #[derive(Debug, Error, ErrorEnum)]
    pub enum UserDomainError {
        #[error(transparent)]
        UnexpectedError(anyhow::Error),
        #[error(transparent)]
        #[without_anyhow]
        ProfileDomainError(ProfileDomainError),
        #[error("invalid-email")]
        InvalidEmail,
        #[error("password-too-short")]
        PasswordTooShort,
        #[error("password-too-long")]
        PasswordTooLong,
        #[error("password-wrong")]
        PasswordWrong
    }

    lazy_static! {
        static ref EMAIL_REGEX: Regex =
        Regex::new("^[a-zA-Z0-9_+&*-]+(?:\\.[a-zA-Z0-9_+&*-]+)*@(?:[a-zA-Z0-9-]+\\.)+[a-zA-Z]{2,}$").unwrap();
    }

    impl User {
        pub fn register(email: String, password: String, username: String) -> Result<(Self, Profile), UserDomainError> {
            Self::validate_email(&email)?;
            Self::validate_password(&password)?;

            let id = Uuid::new_v4().to_string();

            let password = Self::hash_password(&password)?;

            let user = Self {
                id: id.to_string(),
                email,
                password,
                role: "user".to_string(),
            };

            let profile = Profile::register(username, id)?;

            Ok((user, profile))
        }

        pub fn login(&self, password: &str) -> Result<(), UserDomainError> {
            Self::verify_password(&self.password, password)
        }

        // Valid email test (OWASP Regex + maximum length of 60 graphemes
        pub fn validate_email(email: &str) -> Result<(), UserDomainError> {
            let graphemes = email.graphemes(true);
            let mut count = 0;
            for _ in graphemes {
                count += 1;
                if count > 60 {
                    return Err(UserDomainError::InvalidEmail);
                }
            }
            if count < 3 {
                return Err(UserDomainError::InvalidEmail);
            }

            if !EMAIL_REGEX.is_match(email) {
                return Err(UserDomainError::InvalidEmail);
            }

            Ok(())
        }

        pub fn validate_password(password: &str) -> Result<(), UserDomainError> {
            let password_length = password.graphemes(true).count();

            if password_length < 8 {
                return Err(UserDomainError::PasswordTooShort);
            }

            if password_length > 128 {
                return Err(UserDomainError::PasswordTooLong);
            }

            Ok(())
        }

        pub fn get_id(&self) -> String {
            self.id.clone()
        }

        pub fn get_email(&self) -> &str {
            &self.email
        }

        pub fn get_password(&self) -> &str {
            &self.password
        }

        pub fn get_role(&self) -> &str {
            &self.role
        }

        pub fn reset_password(&mut self, old_password: &str, new_password: &str)
                              -> Result<PasswordChangedEvent, UserDomainError> {
            Self::verify_password(&self.password, old_password)?;
            Self::validate_password(&new_password)?;

            self.password = Self::hash_password(&new_password)?;

            let datetime_changed = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|e| UserDomainError::UnexpectedError(e.into()))?
                .as_millis();

            Ok(PasswordChangedEvent::new(self.id.clone(), datetime_changed))
        }

        fn hash_password(cleartext_password: &str) -> Result<String, UserDomainError> {
            let password_salt = SaltString::generate(&mut OsRng);
            ARGON2_HASHER
                .hash_password(cleartext_password.as_ref(), &password_salt)
                .map(|hash| hash.to_string())
                .map_err(|e| UserDomainError::UnexpectedError(e.into()))
        }

        fn verify_password(password_hash: &str, password_cleartext: &str) -> Result<(), UserDomainError> {
            let parsed_hash = PasswordHash::new(password_hash)
                .map_err(|e| UserDomainError::UnexpectedError(e.into()))?;

            ARGON2_HASHER
                .verify_password(password_cleartext.as_bytes(), &parsed_hash)
                .map_err(|e| match e {
                    Error::Password => UserDomainError::PasswordWrong,
                    _ => UserDomainError::UnexpectedError(e.into())
                })
        }
    }
}
