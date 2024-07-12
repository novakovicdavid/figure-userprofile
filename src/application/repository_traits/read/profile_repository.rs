use async_trait::async_trait;

use crate::application::errors::RepositoryError;
use crate::domain::Profile;

#[async_trait]
pub trait ProfileRepository: Send + Sync {
    async fn create(&self, profile: &Profile) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, profile_id: String) -> Result<Profile, RepositoryError>;
    async fn find_by_user_id(&self, user_id: String) -> Result<Profile, RepositoryError>;
    async fn update_profile_by_id(&self, profile_id: String, display_name: Option<String>, bio: Option<String>) -> Result<(), RepositoryError>;
    async fn get_total_profiles_count(&self) -> Result<i64, RepositoryError>;
}