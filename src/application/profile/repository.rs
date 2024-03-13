use async_trait::async_trait;
use figure_lib::rdbs::transaction::TransactionTrait;

use crate::application::errors::RepositoryError;
use crate::domain::Profile;

#[async_trait]
pub trait ProfileRepositoryTrait<T: TransactionTrait>: Send + Sync {
    async fn create(&self, transaction: Option<&mut T>, profile: &Profile) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, transaction: Option<&mut T>, profile_id: String) -> Result<Profile, RepositoryError>;
    async fn find_by_user_id(&self, transaction: Option<&mut T>, user_id: String) -> Result<Profile, RepositoryError>;
    async fn update_profile_by_id(&self, transaction: Option<&mut T>, profile_id: String, display_name: Option<String>, bio: Option<String>) -> Result<(), RepositoryError>;
    async fn get_total_profiles_count(&self, transaction: Option<&mut T>) -> Result<i64, RepositoryError>;
}