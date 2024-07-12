pub use profile_repository::PostgresProfileRepository;

mod profile_repository {
    use async_trait::async_trait;
    use deadpool_postgres::{GenericClient, Pool};
    use figure_lib::get_tokio_postgres_executor;
    use figure_lib::rdbs::postgres::tokio_postgres::TokioPostgresTransaction;
    use tokio_postgres::{Client, GenericClient as OtherGenericClient};
    use tokio_postgres::types::ToSql;

    use crate::application::errors::RepositoryError;
    use crate::application::repository_traits::read::profile_repository::ProfileRepository;
    use crate::domain::Profile;
    use crate::infrastructure::database::entities::ProfileEntity;

    #[derive(Clone)]
    pub struct PostgresProfileRepository {
        pool: Pool,
    }

    impl PostgresProfileRepository {
        pub fn new(pool: Pool) -> Self {
            Self {
                pool,
            }
        }
    }

    #[async_trait]
    impl ProfileRepository for PostgresProfileRepository {
        async fn create(&self, profile: &Profile) -> Result<(), RepositoryError> {
            get_tokio_postgres_executor!(|| async { self.pool.get().await }, client, txn, cnn, lock);

            let statement = client.prepare(r#"
            INSERT INTO profile (id, username, user_id)
            VALUES ($1, $2, $3)
            "#).await?;

            client.execute(&statement, &[
                &profile.get_id(),
                &profile.get_username(),
                &profile.get_user_id()
            ]).await?;

            Ok(())
        }

        async fn find_by_id(&self, profile_id: String) -> Result<Profile, RepositoryError> {
            get_tokio_postgres_executor!(|| async { self.pool.get().await }, client, txn, cnn, lock);

            let statement = client.prepare(r#"
            SELECT * FROM profile WHERE id = $1
            "#).await?;

            Self::find_one(client, statement, &[
                &profile_id,
            ]).await
        }

        async fn find_by_user_id(&self, user_id: String) -> Result<Profile, RepositoryError> {
            get_tokio_postgres_executor!(|| async { self.pool.get().await }, client, txn, cnn, lock);

            let statement = client.prepare(r#"
            SELECT * FROM profile WHERE user_id = $1
            "#).await?;

            Self::find_one(client, statement, &[
                &user_id,
            ]).await
        }

        async fn update_profile_by_id(&self, profile_id: String, display_name: Option<String>, bio: Option<String>) -> Result<(), RepositoryError> {
            get_tokio_postgres_executor!(|| async { self.pool.get().await }, client, txn, cnn, lock);

            let statement = client.prepare(r#"
            UPDATE profile
            SET display_name = $1, bio = $2,
            WHERE id = $3
            "#).await?;

            client.execute(&statement, &[
                &display_name,
                &bio,
                &profile_id
            ]).await?;

            Ok(())
        }

        async fn get_total_profiles_count(&self) -> Result<i64, RepositoryError> {
            get_tokio_postgres_executor!(|| async { self.pool.get().await }, client, txn, cnn, lock);

            let statement = client.prepare(r#"
            SELECT count(*) FROM profile
            "#).await?;

            let count = client.query_one(&statement, &[])
                .await?
                .try_get::<usize, i64>(0)?;

            Ok(count)
        }
    }

    impl PostgresProfileRepository {
        async fn find_one(client: &Client,
                          statement: tokio_postgres::Statement,
                          parameters: &[&(dyn ToSql + Sync)]) -> Result<Profile, RepositoryError>
        {
            let result = client.query_opt(&statement, parameters).await?;

            let row = match result {
                Some(row) => row,
                None => return Err(RepositoryError::ResourceNotFound)
            };

            let entity = ProfileEntity::try_from(row)?;

            Ok(entity.into())
        }
    }
}