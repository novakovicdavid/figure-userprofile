use async_trait::async_trait;
use crate::domain::user::user::User;
use crate::infrastructure::traits::TransactionTrait;
use crate::server_errors::ServerError;

#[async_trait]
pub trait UserRepositoryTrait<T: TransactionTrait>: Send + Sync {
    async fn create(&self, transaction: Option<&mut T>, user: User) -> Result<User, ServerError>;
    async fn find_one_by_email(&self, transaction: Option<&mut T>, email: &str) -> Result<User, ServerError>;
    async fn find_by_id(&self, transaction: Option<&mut T>, id: i64) -> Result<User, ServerError>;
}