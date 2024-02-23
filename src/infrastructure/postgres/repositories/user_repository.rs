use async_trait::async_trait;
use sqlx::{Error, FromRow, Pool, Postgres, Row};
use sqlx::postgres::PgRow;

use crate::application::RepositoryError;
use crate::application::transaction::TransactionTrait;
use crate::application::user_profile::repository::UserRepositoryTrait;
use crate::domain::User;
use crate::infrastructure::postgres::transaction::PostgresTransaction;

#[derive(Clone)]
pub struct UserRepository {
    db: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            db: pool
        }
    }
}

#[async_trait]
impl UserRepositoryTrait<PostgresTransaction> for UserRepository {
    async fn create(&self, transaction: Option<&mut PostgresTransaction>, user: &User) -> Result<(), RepositoryError> {
        let query_string = r#"
        INSERT INTO "user" (email password, role)
        VALUES ($1, $2, 'user')
        "#;

        let query = sqlx::query(&query_string)
            .bind(user.get_email())
            .bind(user.get_password());

        let query_result = match transaction {
            Some(transaction) => query.execute(transaction.inner()).await,
            None => query.execute(&self.db).await
        };

        query_result
            .map(|_| ())
            .map_err(|e| {
                match e {
                    Error::Database(e) => {
                        if e.constraint().is_some() {
                            return RepositoryError::ConstraintConflict;
                        }
                        RepositoryError::UnexpectedError(e.into())
                    }
                    _ => RepositoryError::UnexpectedError(e.into())
                }
            })
    }

    async fn find_one_by_email(&self, transaction: Option<&mut PostgresTransaction>, email: &str) -> Result<User, RepositoryError> {
        let query_string = r#"
        SELECT id, email, password, role
        FROM "user"
        WHERE email = $1
        "#;

        let query =
            sqlx::query_as::<_, User>(&query_string)
                .bind(email);

        let query_result = match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        };

        query_result.map_err(|e| {
            match e {
                Error::RowNotFound => RepositoryError::ResourceNotFound,
                _ => RepositoryError::UnexpectedError(e.into())
            }
        })
    }

    async fn find_by_id(&self, transaction: Option<&mut PostgresTransaction>, id: i64) -> Result<User, RepositoryError> {
        let query_string = r#"
        SELECT id, email, password, role
        FROM "user"
        WHERE id = $1
        "#;

        let query =
            sqlx::query_as::<_, User>(&query_string)
                .bind(id);

        let query_result = match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        };

        query_result.map_err(|e| {
            match e {
                Error::RowNotFound => RepositoryError::ResourceNotFound,
                _ => RepositoryError::UnexpectedError(e.into())
            }
        })
    }
}

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: String = row.try_get("u_user_id")
            .or_else(|_| row.try_get("id"))?;
        let email: String = row.try_get("email")?;
        let password: String = row.try_get("password")?;
        let role: String = row.try_get("role")?;

        Ok(User::new_raw(id, email, password, role))
    }
}
