pub use user_repository::PostgresTokioUserRepository;

mod user_repository {
    use async_trait::async_trait;
    use deadpool_postgres::Pool;
    use figure_lib::rdbs::postgres::tokio_postgres::TokioPostgresTransaction;
    use tokio_postgres::GenericClient;

    use crate::application::errors::RepositoryError;
    use crate::application::user_profile::repository::UserRepositoryTrait;
    use crate::domain::User;
    use crate::infrastructure::postgres::entities::UserEntity;

    pub struct PostgresTokioUserRepository {
        pool: Pool,
    }

    impl PostgresTokioUserRepository {
        pub fn new(pool: Pool) -> Self {
            Self { pool }
        }
    }

    #[async_trait]
    impl UserRepositoryTrait for PostgresTokioUserRepository {
        async fn insert(&self, user: &User) -> Result<(), RepositoryError> {
            let transaction = TokioPostgresTransaction::get_current_transaction();
            let conn;

            let lock;
            let executor = match &transaction {
                Some(txn) => {
                    lock = txn.lock().await;
                    lock.client()
                }
                None => {
                    conn = self.pool.get().await?;
                    conn.client()
                }
            };

            let statement = executor.prepare(r#"
            INSERT INTO "user" (id, email, password, role)
            VALUES ($1, $2, $3, $4)
            "#).await?;

            executor.execute(&statement, &[
                &user.get_id(),
                &user.get_email(),
                &user.get_password(),
                &user.get_role(),
            ]).await?;

            Ok(())
        }

        async fn find_one_by_email(&self, email: &str) -> Result<User, RepositoryError> {
            let transaction = TokioPostgresTransaction::get_current_transaction();
            let conn;

            let lock;
            let executor = match &transaction {
                Some(txn) => {
                    lock = txn.lock().await;
                    lock.client()
                }
                None => {
                    conn = self.pool.get().await?;
                    conn.client()
                }
            };

            let statement = executor.prepare(r#"
            SELECT id, email, password, role
            FROM "user"
            WHERE email = $1
            "#).await?;

            let result = executor.query_opt(&statement, &[
                &email
            ]).await?;

            let row = match result {
                Some(row) => row,
                None => return Err(RepositoryError::ResourceNotFound)
            };

            let entity = UserEntity::try_from(row)?;

            Ok(entity.into())
        }

        async fn find_by_id(&self, id: i64) -> Result<User, RepositoryError> {
            let transaction = TokioPostgresTransaction::get_current_transaction();
            let conn;

            let lock;
            let executor = match &transaction {
                Some(txn) => {
                    lock = txn.lock().await;
                    lock.client()
                }
                None => {
                    conn = self.pool.get().await?;
                    conn.client()
                }
            };

            let statement = executor.prepare(r#"
            SELECT * FROM profile WHERE id = $1
            "#).await?;

            let result = executor.query_opt(&statement, &[
                &id
            ]).await?;

            let row = match result {
                Some(row) => row,
                None => return Err(RepositoryError::ResourceNotFound)
            };

            let entity = UserEntity::try_from(row)?;

            Ok(entity.into())
        }

        async fn set_password(&self, user: &User) -> Result<(), RepositoryError> {
            let transaction = TokioPostgresTransaction::get_current_transaction();
            let conn;

            let lock;
            let executor = match &transaction {
                Some(txn) => {
                    lock = txn.lock().await;
                    lock.client()
                }
                None => {
                    conn = self.pool.get().await?;
                    conn.client()
                }
            };

            let statement = executor.prepare(r#"
            UPDATE "user"
            SET password = $1
            WHERE id = $2
            "#).await?;

            executor.query_opt(&statement, &[
                &user.password,
                &user.id
            ]).await?;

            Ok(())
        }
    }
}