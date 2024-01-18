use async_trait::async_trait;

#[async_trait]
pub trait TransactionManagerTrait<T: TransactionTrait>: Send + Sync {
    async fn create(&self) -> Result<T, TransactionError>;
}

#[async_trait]
pub trait TransactionTrait: Send + Sync {
    type Inner;
    async fn commit(self) -> Result<(), TransactionError>;
    fn inner(&mut self) -> &mut Self::Inner;
}

#[derive(Debug)]
pub enum TransactionError {
    UnexpectedError(anyhow::Error),
}