use async_trait::async_trait;
use crate::domain::profile::profile::Profile;
use crate::application::transaction::TransactionTrait;
use crate::application::server_errors::ServerError;

#[async_trait]
pub trait ProfileRepositoryTrait<T: TransactionTrait>: Send + Sync {
    async fn create(&self, transaction: Option<&mut T>, profile: Profile) -> Result<Profile, ServerError>;
    async fn find_by_id(&self, transaction: Option<&mut T>, profile_id: i64) -> Result<Profile, ServerError>;
    async fn find_by_user_id(&self, transaction: Option<&mut T>, user_id: i64) -> Result<Profile, ServerError>;
    async fn update_profile_by_id(&self, transaction: Option<&mut T>, profile_id: i64, display_name: Option<String>, bio: Option<String>) -> Result<(), ServerError>;
    async fn get_total_profiles_count(&self, transaction: Option<&mut T>) -> Result<i64, ServerError>;
}