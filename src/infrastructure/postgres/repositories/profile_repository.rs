pub use profile_repository::ProfileRepository;

mod profile_repository {
    use async_trait::async_trait;
    use sqlx::{Pool, Postgres, Row};

    use crate::application::errors::RepositoryError;
    use crate::application::profile::repository::ProfileRepositoryTrait;
    use crate::application::transaction::TransactionTrait;
    use crate::domain::Profile;
    use crate::infrastructure::postgres::transaction::PostgresTransaction;

    #[derive(Clone)]
    pub struct ProfileRepository {
        db: Pool<Postgres>,
    }

    impl ProfileRepository {
        pub fn new(pool: Pool<Postgres>) -> Self {
            Self {
                db: pool
            }
        }
    }

    #[async_trait]
    impl ProfileRepositoryTrait<PostgresTransaction> for ProfileRepository {
        async fn create(&self, transaction: Option<&mut PostgresTransaction>, profile: &Profile) -> Result<(), RepositoryError> {
            let query_string = r#"
        INSERT INTO profile (username, user_id)
        VALUES ($1, $2)
        RETURNING id, username, user_id
        "#;

            let query = sqlx::query(&query_string)
                .bind(profile.get_username())
                .bind(profile.get_user_id());

            let query_result = match transaction {
                Some(transaction) => query.fetch_one(transaction.inner()).await,
                None => query.fetch_one(&self.db).await
            };

            query_result
                .map(|_| ())
                .map_err(|e| {
                    match e {
                        sqlx::Error::Database(e) => {
                            if e.constraint().is_some() {
                                return RepositoryError::ConstraintConflict;
                            }
                            RepositoryError::UnexpectedError(e.into())
                        }
                        _ => RepositoryError::UnexpectedError(e.into())
                    }
                })
        }

        async fn find_by_id(&self, transaction: Option<&mut PostgresTransaction>, profile_id: String) -> Result<Profile, RepositoryError> {
            let query_string = "SELECT * FROM profile WHERE id = $1";

            let query =
                sqlx::query_as::<_, Profile>(&query_string)
                    .bind(profile_id);

            let query_result = match transaction {
                Some(transaction) => query.fetch_one(transaction.inner()).await,
                None => query.fetch_one(&self.db).await
            };

            match query_result {
                Ok(profile) => Ok(profile),
                Err(sqlx::Error::RowNotFound) => Err(RepositoryError::ResourceNotFound),
                Err(e) => Err(RepositoryError::UnexpectedError(e.into()))
            }
        }

        async fn find_by_user_id(&self, transaction: Option<&mut PostgresTransaction>, user_id: String) -> Result<Profile, RepositoryError> {
            let query_string = "SELECT * FROM profile WHERE user_id = $1";

            let query =
                sqlx::query_as::<_, Profile>(&query_string)
                    .bind(user_id);

            let query_result = match transaction {
                Some(transaction) => query.fetch_one(transaction.inner()).await,
                None => query.fetch_one(&self.db).await
            };

            match query_result {
                Ok(profile) => Ok(profile),
                Err(sqlx::Error::RowNotFound) => Err(RepositoryError::ResourceNotFound),
                Err(e) => Err(RepositoryError::UnexpectedError(e.into()))
            }
        }

        async fn update_profile_by_id(&self, transaction: Option<&mut PostgresTransaction>, profile_id: String, display_name: Option<String>, bio: Option<String>) -> Result<(), RepositoryError> {
            let query_string = r#"
        UPDATE profile
        SET display_name = $1, bio = $2,
        WHERE id = $5
        "#;

            let query =
                sqlx::query(&query_string)
                    .bind(display_name)
                    .bind(bio)
                    .bind(profile_id);

            match transaction {
                Some(transaction) => query.execute(transaction.inner()).await,
                None => query.execute(&self.db).await
            }
                .map(|_result| ())
                .map_err(|e| RepositoryError::UnexpectedError(e.into()))
        }

        async fn get_total_profiles_count(&self, transaction: Option<&mut PostgresTransaction>) -> Result<i64, RepositoryError> {
            let query_string = "SELECT count(*) FROM profile";

            let query = sqlx::query(&query_string);

            match transaction {
                Some(transaction) => query.fetch_one(transaction.inner()).await,
                None => query.fetch_one(&self.db).await
            }
                .and_then(|row| row.try_get(0))
                .map_err(|e| RepositoryError::UnexpectedError(e.into()))
        }
    }
}