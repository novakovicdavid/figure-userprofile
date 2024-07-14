use async_trait::async_trait;
use deadpool_postgres::Pool;
use figure_lib::get_tokio_postgres_executor;
use figure_lib::rdbs::postgres::sea_query_misc::{Column, Table};
use figure_lib::rdbs::postgres::tokio_postgres::TokioPostgresTransaction;
use sea_query::{PostgresQueryBuilder, Query};
use sea_query_postgres::PostgresBinder;
use tokio_postgres::{GenericClient, Row};

use crate::application::errors::RepositoryError;
use crate::application::repository_traits::read::user_repository::UserRepository;
use crate::domain::User;
use crate::domain::user::user::ResetPasswordRequest;
use crate::infrastructure::database::entities::{ResetPasswordRequestEntity, UserEntity};

#[derive(Clone)]
pub struct TokioPostgresUserRepository {
        pool: Pool,
    }

impl TokioPostgresUserRepository {
        pub fn new(pool: Pool) -> Self {
            Self { pool }
        }
    }

    #[async_trait]
    impl UserRepository for TokioPostgresUserRepository {
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
            FOR UPDATE
            "#).await?;

            let row = client.query_opt(&statement, &[&email]).await?
                .ok_or_else(|| RepositoryError::ResourceNotFound)?;

            let entity = UserEntity::try_from(row)?;

            let password_resets_statement = client.prepare(r#"
            SELECT user_id, token, datetime FROM password_reset_request
            WHERE user_id = $1
            ORDER BY password_reset_request.datetime
            FOR UPDATE
            "#).await?;

            let password_reset_requests_rows = client
                .query(&password_resets_statement, &[&entity.id])
                .await?;

            let password_reset_requests = Self
            ::process_password_reset_request_rows(password_reset_requests_rows).await?;

            let user = entity.into_user(password_reset_requests);

            Ok(user)
        }

        async fn find_by_id(&self, user_id: &str) -> Result<User, RepositoryError> {
            get_tokio_postgres_executor!(|| async { self.pool.get().await }, client, txn, cnn, lock);

            let statement = client.prepare(r#"
            SELECT
            id, email, password, role
            FROM "user"
            WHERE id = $1
            FOR UPDATE
            "#).await?;

            let row = client.query_opt(&statement, &[&user_id]).await?
                .ok_or_else(|| RepositoryError::ResourceNotFound)?;
            ;

            let entity = UserEntity::try_from(row)?;

            let password_resets_statement = client.prepare(r#"
            SELECT user_id, token, datetime FROM password_reset_request
            WHERE user_id = $1
            ORDER BY password_reset_request.datetime
            FOR UPDATE
            "#).await?;

            let password_reset_requests_rows = client
                .query(&password_resets_statement, &[&user_id])
                .await?;

            let password_reset_requests = Self
            ::process_password_reset_request_rows(password_reset_requests_rows).await?;

            let user = entity.into_user(password_reset_requests);

            Ok(user)
        }

        async fn update(&self, user: &User) -> Result<(), RepositoryError> {
            get_tokio_postgres_executor!(|| async { self.pool.get().await }, client, txn, cnn, lock);

            let statement = client.prepare(r#"
            UPDATE "user"
            SET email = $2, password = $3, role = $4
            WHERE id = $1
            "#).await?;

            client.execute(&statement, &[
                &user.get_id(),
                &user.get_email(),
                &user.get_password(),
                &user.get_role()
            ]).await?;

            let statement = client.prepare(r#"
            DELETE FROM password_reset_request
            WHERE user_id = $1
            "#).await?;

            client.execute(&statement, &[&user.get_id()]).await?;

            if user.password_reset_requests().len() > 0 {
                let mut insert = Query::insert();
                let mut statement = insert.into_table(Table("password_reset_request"))
                    .columns([Column("user_id"), Column("token"), Column("datetime")]);

                for password_reset_request in user.password_reset_requests() {
                    statement = statement.values(
                        [
                            user.get_id().clone().into(),
                            password_reset_request.token().clone().into(),
                            password_reset_request.datetime().clone().into()
                        ]
                    )?;
                }

                let (statement2, values) = statement.build_postgres(PostgresQueryBuilder);

                client.execute(&statement2, &values.as_params()).await?;
            }

            Ok(())
        }

        async fn find_by_reset_password_token(&self, token: &str) -> Result<User, RepositoryError> {
            get_tokio_postgres_executor!(|| async { self.pool.get().await }, client, txn, cnn, lock);

            let statement = client.prepare(r#"
            SELECT id, email, password, role
            FROM "user"
            INNER JOIN password_reset_request ON "user".id = password_reset_request.user_id
            WHERE password_reset_request.token = $1
            ORDER BY password_reset_request.datetime
            FOR UPDATE
            "#).await?;

            let row = client.query_opt(&statement, &[&token]).await?
                .ok_or_else(|| RepositoryError::ResourceNotFound)?;
            let entity = UserEntity::try_from(row)?;

            let password_resets_statement = client.prepare(r#"
            SELECT user_id, token, datetime FROM password_reset_request
            WHERE user_id = $1
            FOR UPDATE
            "#).await?;

            let password_reset_requests_rows = client
                .query(&password_resets_statement, &[&entity.id])
                .await?;

            let password_reset_requests = Self
            ::process_password_reset_request_rows(password_reset_requests_rows).await?;

            let user = entity.into_user(password_reset_requests);

            Ok(user)
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