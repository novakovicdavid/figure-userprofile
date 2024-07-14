use async_trait::async_trait;

use crate::application::errors::ApplicationError;

#[async_trait]
pub trait MigrationRunner: Send + Sync {
    async fn run(&self) -> Result<(), ApplicationError>;
}