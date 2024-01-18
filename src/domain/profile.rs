pub use profile::Profile;
pub use profile::ProfileDomainError;

pub mod profile {
    use lazy_static::lazy_static;
    use regex::Regex;
    use unicode_segmentation::UnicodeSegmentation;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Profile {
        id: i64,
        username: String,
        display_name: Option<String>,
        bio: Option<String>,
        banner: Option<String>,
        profile_picture: Option<String>,
        user_id: i64,
    }

    #[derive(Debug)]
    pub enum ProfileDomainError {
        InvalidUsername,
    }

    lazy_static! {
        static ref USERNAME_REGEX: Regex =
        Regex::new("^[a-zA-Z0-9]+-*[a-zA-Z0-9]+?$").unwrap();
    }

    impl Profile {
        pub fn new(id: i64, username: String, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>, user_id: i64) -> Result<Self, ProfileDomainError> {
            Self::validate_username(&username)?;

            Ok(Self::new_raw(id, username, display_name, bio, banner, profile_picture, user_id))
        }

        pub fn new_raw(id: i64, username: String, display_name: Option<String>, bio: Option<String>, banner: Option<String>, profile_picture: Option<String>, user_id: i64) -> Self {
            Self {
                id,
                username,
                display_name,
                bio,
                banner,
                profile_picture,
                user_id,
            }
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

        pub fn get_id(&self) -> i64 {
            self.id
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

        pub fn get_user_id(&self) -> i64 {
            self.user_id
        }

        pub fn set_id(&mut self, id: i64) {
            self.id = id;
        }

        pub fn set_username(&mut self, username: String) {
            self.username = username;
        }

        pub fn set_display_name(&mut self, display_name: Option<String>) {
            self.display_name = display_name
        }

        pub fn set_bio(&mut self, bio: Option<String>) {
            self.bio = bio;
        }

        pub fn set_banner(&mut self, banner: Option<String>) {
            self.banner = banner;
        }

        pub fn set_profile_picture(&mut self, profile_picture: Option<String>) {
            self.profile_picture = profile_picture;
        }

        pub fn set_user_id(&mut self, user_id: i64) {
            self.user_id = user_id;
        }
    }
}