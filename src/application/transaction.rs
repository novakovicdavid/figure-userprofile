use async_trait::async_trait;
use thiserror::Error;

#[async_trait]
pub trait TransactionManagerTrait<T: TransactionTrait>: Send + Sync {
    async fn create(&self) -> Result<T, TransactionError>;
}

#[async_trait]
pub trait TransactionTrait: 'static + Send + Sync {
    type Inner;
    async fn commit(self) -> Result<(), TransactionError>;
    fn inner(&mut self) -> &mut Self::Inner;
}

#[derive(Debug, Error)]
pub enum TransactionError {
    #[error(transparent)]
    UnexpectedError(anyhow::Error),
}