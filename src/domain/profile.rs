pub use profile::Profile;
pub use profile::ProfileDomainError;

pub mod profile {
    use lazy_static::lazy_static;
    use regex::Regex;
    use thiserror::Error;
    use unicode_segmentation::UnicodeSegmentation;
    use uuid::Uuid;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Profile {
        pub id: String,
        pub username: String,
        pub display_name: Option<String>,
        pub bio: Option<String>,
        pub banner: Option<String>,
        pub profile_picture: Option<String>,
        pub user_id: String,
    }

    #[derive(Debug, Error)]
    pub enum ProfileDomainError {
        #[error("invalid-username")]
        InvalidUsername,
    }

    lazy_static! {
        static ref USERNAME_REGEX: Regex =
        Regex::new("^[a-zA-Z0-9]+-*[a-zA-Z0-9]+?$").unwrap();
    }

    impl Profile {
        pub fn register(username: String, user_id: String) -> Result<Self, ProfileDomainError> {
            Self::validate_username(&username)?;

            let id = Uuid::new_v4().to_string();

            Ok(Self {
                id,
                username,
                display_name: None,
                bio: None,
                banner: None,
                profile_picture: None,
                user_id,
            })
        }

        // Valid username test
        // (alphanumerical, optionally a dash surrounded by alphanumerical characters, 15 character limit)
        pub fn validate_username(username: &str) -> Result<(), ProfileDomainError> {
            let username_count = username.graphemes(true).count();

            if !USERNAME_REGEX.is_match(username) || !(3..=15).contains(&username_count) {
                return Err(ProfileDomainError::InvalidUsername);
            }

            Ok(())
        }

        pub fn get_id(&self) -> String {
            self.id.clone()
        }

        pub fn get_username(&self) -> &str {
            &self.username
        }

        pub fn get_display_name(&self) -> &Option<String> {
            &self.display_name
        }

        pub fn get_bio(&self) -> &Option<String> {
            &self.bio
        }

        pub fn get_banner(&self) -> &Option<String> {
            &self.banner
        }

        pub fn get_profile_picture(&self) -> &Option<String> {
            &self.profile_picture
        }

        pub fn get_user_id(&self) -> String {
            self.user_id.clone()
        }
    }
}