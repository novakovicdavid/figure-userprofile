use async_trait::async_trait;
use deadpool_postgres::Pool;
use figure_lib::get_tokio_postgres_executor;
use figure_lib::rdbs::postgres::tokio_postgres::TokioPostgresTransaction;
use tokio_postgres::{GenericClient, Row};

use crate::application::errors::RepositoryError;
use crate::application::repository_traits::user_repository::UserRepositoryTrait;
use crate::domain::User;
use crate::domain::user::user::ResetPasswordRequest;
use crate::infrastructure::database::entities::{ResetPasswordRequestEntity, UserEntity};

pub struct TokioPostgresUserRepository {
        pool: Pool,
    }

impl TokioPostgresUserRepository {
        pub fn new(pool: Pool) -> Self {
            Self { pool }
        }
    }

    #[async_trait]
    impl UserRepositoryTrait for TokioPostgresUserRepository {
        async fn insert(&self, user: &User) -> Result<(), RepositoryError> {
            get_tokio_postgres_executor!(|| async { self.pool.get().await }, client, txn, cnn, lock);

            let user_statement = client.prepare(r#"
            INSERT INTO "user" (id, email, password, role)
            VALUES ($1, $2, $3, $4)
            "#).await?;

            client.execute(&user_statement, &[
                &user.get_id(),
                &user.get_email(),
                &user.get_password(),
                &user.get_role(),
            ]).await?;

            Ok(())
        }

        async fn find_one_by_email(&self, email: &str) -> Result<User, RepositoryError> {
            get_tokio_postgres_executor!(|| async { self.pool.get().await }, client, txn, cnn, lock);

            let statement = client.prepare(r#"
            SELECT
            id, email, password, role
            FROM "user"
            WHERE email = $1
            "#).await?;

            let result = client.query_opt(&statement, &[
                &email
            ]).await?;

            let row = match result {
                Some(row) => row,
                None => return Err(RepositoryError::ResourceNotFound)
            };

            let entity = UserEntity::try_from(row)?;
            let mut user = User::from(entity);

            let password_resets_statement = client.prepare(r#"
            SELECT user_id, token, datetime FROM password_reset_request
            WHERE user_id = $1
            FOR UPDATE
            "#).await?;

            let password_reset_requests_rows = client
                .query(&password_resets_statement, &[&user.id])
                .await?;

            let password_reset_requests = Self
            ::process_password_reset_request_rows(password_reset_requests_rows).await?;

            for password_reset in password_reset_requests {
                user.password_resets.push(password_reset);
            }

            Ok(user)
        }

        async fn find_by_id(&self, user_id: &str) -> Result<User, RepositoryError> {
            get_tokio_postgres_executor!(|| async { self.pool.get().await }, client, txn, cnn, lock);

            let password_resets_statement = client.prepare(r#"
            SELECT user_id, token, datetime FROM password_reset_request
            WHERE user_id = $1
            FOR UPDATE
            "#).await?;

            let password_reset_requests_rows = client
                .query(&password_resets_statement, &[&user_id])
                .await?;

            let password_reset_requests = Self
            ::process_password_reset_request_rows(password_reset_requests_rows).await?;

            let statement = client.prepare(r#"
            SELECT * FROM profile WHERE id = $1
            "#).await?;

            let result = client
                .query_opt(&statement, &[&user_id])
                .await?;

            let row = match result {
                Some(row) => row,
                None => return Err(RepositoryError::ResourceNotFound)
            };

            let entity = UserEntity::try_from(row)?;

            let mut user: User = entity.into();

            for password_reset in password_reset_requests {
                user.password_resets.push(password_reset);
            }

            Ok(user)
        }

        async fn save(&self, user: &User) -> Result<(), RepositoryError> {
            get_tokio_postgres_executor!(|| async { self.pool.get().await }, client, txn, cnn, lock);

            let statement = client.prepare(r#"
            UPDATE "user"
            SET email = $2, password = $3, role = $4
            WHERE id = $1
            "#).await?;

            client.execute(&statement, &[
                &user.id,
                &user.email,
                &user.password,
                &user.role
            ]).await?;

            Ok(())
        }
    }

impl TokioPostgresUserRepository {
    async fn process_password_reset_request_rows(rows: Vec<Row>) -> Result<Vec<ResetPasswordRequest>, RepositoryError> {
        let password_reset_requests = rows
            .into_iter()
            .map(|row| ResetPasswordRequestEntity::try_from(row))
            .collect::<Vec<_>>();

        let mut vec = Vec::new();

        for password_reset in password_reset_requests {
            vec.push(ResetPasswordRequest::from(password_reset?));
        }

        Ok(vec)
    }
}