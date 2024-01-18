use async_trait::async_trait;
use sqlx::{PgConnection, Pool, Postgres, Transaction};

use crate::application::transaction::{TransactionError, TransactionManagerTrait, TransactionTrait};

#[derive(Clone)]
pub struct PostgresTransactionManager {
    db: Pool<Postgres>
}

impl PostgresTransactionManager {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self {
            db
        }
    }
}

#[async_trait]
impl TransactionManagerTrait<PostgresTransaction> for PostgresTransactionManager {
    async fn create(&self) -> Result<PostgresTransaction, TransactionError> {
        self.db.begin().await
            .map(PostgresTransaction::new)
            .map_err(|e| TransactionError::UnexpectedError(e.into()))
    }
}

pub struct PostgresTransaction {
    transaction: Transaction<'static, Postgres>
}

impl PostgresTransaction {
    pub fn new(transaction: Transaction<'static, Postgres>) -> Self {
        Self {
            transaction
        }
    }
}

#[async_trait]
impl TransactionTrait for PostgresTransaction {
    type Inner = PgConnection;
    async fn commit(self) -> Result<(), TransactionError> {
        self.transaction.commit().await
            .map_err(|e| TransactionError::UnexpectedError(e.into()))
    }

    fn inner(&mut self) -> &mut Self::Inner {
        &mut *self.transaction
    }
}