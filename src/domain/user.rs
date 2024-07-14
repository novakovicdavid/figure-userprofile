pub use user::User;
pub use user::UserDomainError;

pub mod user {
    use std::ops::Sub;
    use std::time::Duration;

    use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
    use argon2::password_hash::{Error, SaltString};
    use error_conversion_macro::ErrorEnum;
    use lazy_static::lazy_static;
    use rand_chacha::ChaCha20Rng;
    use rand_core::{OsRng, RngCore, SeedableRng};
    use regex::Regex;
    use thiserror::Error;
    use time::OffsetDateTime;
    use unicode_segmentation::UnicodeSegmentation;
    use uuid::Uuid;

    use crate::application::domain_event_dispatcher::{DomainEvent, PasswordResetRequested};
    use crate::domain::Profile;
    use crate::domain::profile::ProfileDomainError;
    use crate::infrastructure::secure_hasher::ARGON2_HASHER;

    pub struct User {
        id: String,
        email: String,
        password: String,
        role: String,
        password_reset_requests: Vec<ResetPasswordRequest>,
    }

    pub struct ResetPasswordRequest {
        token: String,
        datetime: OffsetDateTime,
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
        PasswordWrong,
        #[error("too-many-password-resets-requested")]
        TooManyPasswordResetsRequested,
        #[error("invalid-password-reset-token")]
        InvalidPasswordResetToken,
        #[error("password-reset-token-expired")]
        PasswordResetTokenExpired,
    }

    lazy_static! {
        static ref EMAIL_REGEX: Regex =
        Regex::new("^[a-zA-Z0-9_+&*-]+(?:\\.[a-zA-Z0-9_+&*-]+)*@(?:[a-zA-Z0-9-]+\\.)+[a-zA-Z]{2,}$").unwrap();
    }

    impl User {
        pub fn new(id: String, email: String, password: String, role: String,
                   password_reset_requests: Vec<ResetPasswordRequest>) -> Self {
            Self { id, email, password, role, password_reset_requests }
        }

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
                password_reset_requests: Vec::new()
            };

            let profile = Profile::register(username, id)?;

            Ok((user, profile))
        }

        pub fn login(&self, password: &str) -> Result<(), UserDomainError> {
            Self::verify_password(&self.password, password)
        }

        pub fn request_password_reset(&mut self, requester: String) -> Result<DomainEvent, UserDomainError> {
            let mut recent_requests = 0;

            let datetime_now = OffsetDateTime::now_utc();
            let datetime_one_hour_ago = datetime_now
                .sub(Duration::from_secs(60 * 60));

            for request in &self.password_reset_requests {
                if datetime_one_hour_ago.unix_timestamp() < request.datetime.unix_timestamp() {
                    recent_requests += 1;
                }

                if recent_requests == 3 {
                    return Err(UserDomainError::TooManyPasswordResetsRequested);
                }
            }

            let token = ChaCha20Rng::from_entropy().next_u64().to_string();

            self.password_reset_requests.push(ResetPasswordRequest {
                token: token.clone(),
                datetime: datetime_now,
            });

            Ok(PasswordResetRequested {
                token,
                email: self.email.clone(),
                requester,
                datetime: OffsetDateTime::now_utc(),
            }.into())
        }

        pub fn reset_password_using_password_reset_token(&mut self, supplied_token: &str, new_password: &str) -> Result<(), UserDomainError> {
            let found_token = match self.password_reset_requests
                .iter().find(|request| request.token == supplied_token) {
                None => return Err(UserDomainError::InvalidPasswordResetToken),
                Some(token) => token
            };

            let one_hour_ago = OffsetDateTime::now_utc()
                .sub(Duration::from_secs(60 * 60));

            if found_token.datetime.unix_timestamp() < one_hour_ago.unix_timestamp() {
                return Err(UserDomainError::PasswordResetTokenExpired);
            }

            Self::validate_password(&new_password)?;
            let new_password = Self::hash_password(&new_password)?;
            self.password = new_password;

            self.password_reset_requests.clear();

            // todo domain event
            Ok(())
        }

        // Valid email test (OWASP Regex + maximum length of 60 graphemes)
        // todo unit tests
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

        // todo unit tests
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

        // todo unit tests
        fn hash_password(cleartext_password: &str) -> Result<String, UserDomainError> {
            let password_salt = SaltString::generate(&mut OsRng);
            ARGON2_HASHER
                .hash_password(cleartext_password.as_ref(), &password_salt)
                .map(|hash| hash.to_string())
                .map_err(|e| UserDomainError::UnexpectedError(e.into()))
        }

        // todo unit tests
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

        pub fn password_reset_requests(&self) -> &Vec<ResetPasswordRequest> {
            &self.password_reset_requests
        }
    }

    impl ResetPasswordRequest {
        pub fn new(token: String, datetime: OffsetDateTime) -> Self {
            Self { token, datetime }
        }

        pub fn token(&self) -> &str {
            &self.token
        }

        pub fn datetime(&self) -> OffsetDateTime {
            self.datetime
        }
    }
}
