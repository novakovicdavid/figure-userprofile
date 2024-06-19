use async_trait::async_trait;

use crate::application::errors::RepositoryError;
use crate::domain::user::user::User;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn insert(&self, user: &User) -> Result<(), RepositoryError>;
    async fn find_one_by_email(&self, email: &str) -> Result<User, RepositoryError>;
    async fn find_by_id(&self, id: i64) -> Result<User, RepositoryError>;
    async fn update(&self, user: &User) -> Result<(), RepositoryError>;
}