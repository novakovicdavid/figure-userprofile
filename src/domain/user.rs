pub use user::User;
pub use user::UserDomainError;

pub mod user {
    use lazy_static::lazy_static;
    use regex::Regex;
    use thiserror::Error;
    use unicode_segmentation::UnicodeSegmentation;

    #[derive(Debug, Clone, PartialEq)]
    pub struct User {
        id: String,
        email: String,
        password: String,
        role: String,
    }

    #[derive(Debug, Error)]
    pub enum UserDomainError {
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
        pub fn new(id: String, email: String, password: String, role: String) -> Result<Self, UserDomainError> {
            Self::validate_email(&email)?;

            Ok(Self::new_raw(id, email, password, role))
        }

        pub fn new_raw(id: String, email: String, password: String, role: String) -> Self {
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
    }
}
