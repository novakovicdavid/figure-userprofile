pub use user::User;
pub use user::UserError;

pub mod user {
    use lazy_static::lazy_static;
    use regex::Regex;
    use unicode_segmentation::UnicodeSegmentation;

    #[derive(Debug, Clone, PartialEq)]
    pub struct User {
        id: i64,
        email: String,
        password: String,
        role: String,
    }

    #[derive(Debug)]
    pub enum UserError {
        InvalidEmail,
        PasswordTooShort,
        PasswordTooLong,
    }

    lazy_static! {
        static ref EMAIL_REGEX: Regex =
        Regex::new("^[a-zA-Z0-9_+&*-]+(?:\\.[a-zA-Z0-9_+&*-]+)*@(?:[a-zA-Z0-9-]+\\.)+[a-zA-Z]{2,}$").unwrap();
    }

    impl User {
        pub fn new(id: i64, email: String, password: String, role: String) -> Result<Self, UserError> {
            Self::validate_email(&email)?;

            Ok(Self::new_raw(id, email, password, role))
        }

        pub fn new_raw(id: i64, email: String, password: String, role: String) -> Self {
            Self {
                id,
                email: email.to_lowercase(),
                password,
                role,
            }
        }

        // Valid email test (OWASP Regex + maximum length of 60 graphemes
        pub fn validate_email(email: &str) -> Result<(), UserError> {
            let graphemes = email.graphemes(true);
            let mut count = 0;
            for _ in graphemes {
                count += 1;
                if count > 60 {
                    return Err(UserError::InvalidEmail);
                }
            }
            if count < 3 {
                return Err(UserError::InvalidEmail);
            }

            if !EMAIL_REGEX.is_match(email) {
                return Err(UserError::InvalidEmail);
            }

            Ok(())
        }

        pub fn validate_password(password: &str) -> Result<(), UserError> {
            let password_length = password.graphemes(true).count();

            if password_length < 8 {
                return Err(UserError::PasswordTooShort);
            }

            if password_length > 128 {
                return Err(UserError::PasswordTooLong);
            }

            Ok(())
        }

        pub fn get_id(&self) -> i64 {
            self.id
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

        pub fn set_id(&mut self, id: i64) {
            self.id = id;
        }

        pub fn set_email(&mut self, email: String) {
            self.email = email;
        }

        pub fn set_password(&mut self, password: String) {
            self.password = password;
        }

        pub fn set_role(&mut self, role: String) {
            self.role = role;
        }
    }
}
