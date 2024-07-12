use async_trait::async_trait;

use crate::application::errors::RepositoryError;
use crate::domain::user::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn insert(&self, user: &User) -> Result<(), RepositoryError>;
    async fn find_one_by_email(&self, email: &str) -> Result<User, RepositoryError>;
    async fn find_by_id(&self, id: &str) -> Result<User, RepositoryError>;
    async fn save(&self, user: &User) -> Result<(), RepositoryError>;
}