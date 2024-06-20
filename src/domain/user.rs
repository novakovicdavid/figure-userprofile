pub use user::User;
pub use user::UserDomainError;

pub mod user {
    use std::time::SystemTime;

    use error_conversion_macro::ErrorEnum;
    use figure_lib::queue::events::user::PasswordChangedEvent;
    use lazy_static::lazy_static;
    use regex::Regex;
    use thiserror::Error;
    use unicode_segmentation::UnicodeSegmentation;
    use uuid::Uuid;

    use crate::domain::Profile;
    use crate::domain::profile::ProfileDomainError;
    use crate::infrastructure::secure_hasher::SecureHasher;

    #[derive(Debug, Clone, PartialEq)]
    pub struct User {
        id: String,
        email: String,
        password: String,
        role: String,
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

            let user = Self {
                id: id.to_string(),
                email,
                password,
                role: "user".to_string(),
            };

            let profile = Profile::register(username, id)?;

            Ok((user, profile))
        }

        pub fn new_unchecked(id: String, email: String, password: String, role: String) -> Self {
            Self {
                id,
                email: email.to_lowercase(),
                password,
                role,
            }
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

        pub fn set_password(&mut self, password: String, hasher: &Box<dyn SecureHasher>)
                            -> Result<PasswordChangedEvent, UserDomainError> {
            Self::validate_password(&password)?;

            self.password = hasher.hash_password(&password)
                .map_err(|e| UserDomainError::UnexpectedError(e.into()))?;

            let datetime_changed = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|e| UserDomainError::UnexpectedError(e.into()))?
                .as_millis();

            Ok(PasswordChangedEvent::new(self.id.clone(), datetime_changed))
        }
    }
}
