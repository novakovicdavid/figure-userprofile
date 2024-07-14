pub mod postgres_tokio_migration_runner {
    use std::ops::DerefMut;

    use async_trait::async_trait;
    use deadpool_postgres::Pool;

    use crate::application::errors::ApplicationError;
    use crate::application::migration_runner_trait::MigrationRunner;

    mod embedded {
        use refinery::embed_migrations;

        embed_migrations!("./src/infrastructure/database/migrations");
    }

    pub struct TokioPostgresMigrationRunner {
        pool: Pool,
    }

    impl TokioPostgresMigrationRunner {
        pub fn new(pool: Pool) -> Self {
            Self { pool }
        }
    }

    #[async_trait]
    impl MigrationRunner for TokioPostgresMigrationRunner {
        async fn run(&self) -> Result<(), ApplicationError> {
            let mut conn = self.pool.get().await
                .map_err(|e| ApplicationError::UnexpectedError(e.into()))?;
            let client = conn.deref_mut().deref_mut();

            embedded::migrations::runner().run_async(client).await
                .map_err(|e| ApplicationError::UnexpectedError(e.into()))?;

            Ok(())
        }
    }
}