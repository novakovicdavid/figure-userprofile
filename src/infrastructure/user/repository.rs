use async_trait::async_trait;
use sqlx::{Error, FromRow, Pool, Postgres, Row};
use sqlx::postgres::PgRow;
use crate::application::server_errors::ServerError;
use crate::application::user::repository::UserRepositoryTrait;
use crate::domain::user::user::User;
use crate::application::transaction::TransactionTrait;
use crate::infrastructure::transaction::PostgresTransaction;

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
    async fn create(&self, transaction: Option<&mut PostgresTransaction>, user: User) -> Result<User, ServerError> {
        let query_string = r#"
        INSERT INTO "user" (email password, role)
        VALUES ($1, $2, 'user')
        RETURNING id, email, password, role
        "#;

        let query = sqlx::query_as::<_, User>(&query_string)
            .bind(user.get_email())
            .bind(user.get_password());

        match transaction {
            Some(transaction) => query.fetch_one(transaction.inner()).await,
            None => query.fetch_one(&self.db).await
        }
            .map_err(|e| {
                match e {
                    Error::Database(e) => {
                        if e.constraint() == Some("user_email_uindex") {
                            return ServerError::EmailAlreadyInUse;
                        }
                        ServerError::InternalError(e.into())
                    }
                    _ => ServerError::InternalError(e.into())
                }
            })
    }

    async fn find_one_by_email(&self, transaction: Option<&mut PostgresTransaction>, email: &str) -> Result<User, ServerError> {
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
                Error::RowNotFound => ServerError::ResourceNotFound,
                _ => ServerError::InternalError(e.into())
            }
        })
    }

    async fn find_by_id(&self, transaction: Option<&mut PostgresTransaction>, id: i64) -> Result<User, ServerError> {
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
                Error::RowNotFound => ServerError::ResourceNotFound,
                _ => ServerError::InternalError(e.into())
            }
        })
    }
}

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: i64 = row.try_get("u_user_id")
            .or_else(|_| row.try_get("id"))?;
        let email: String = row.try_get("email")?;
        let password: String = row.try_get("password")?;
        let role: String = row.try_get("role")?;

        Ok(User::new_raw(id, email, password, role))
    }
}
