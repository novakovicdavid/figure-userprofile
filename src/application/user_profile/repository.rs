use async_trait::async_trait;
use crate::application::error_handling::RepositoryError;
use crate::domain::user::user::User;

#[async_trait]
pub trait UserRepositoryTrait<T>: Send + Sync {
    async fn create(&self, transaction: Option<&mut T>, user: User) -> Result<User, RepositoryError>;
    async fn find_one_by_email(&self, transaction: Option<&mut T>, email: &str) -> Result<User, RepositoryError>;
    async fn find_by_id(&self, transaction: Option<&mut T>, id: i64) -> Result<User, RepositoryError>;
}