use std::time::Instant;

use sqlx::{Pool, Postgres};
use tokio::task;
use tracing::info;
use url::Url;

use crate::application::profile::service::ProfileService;
use crate::application::transaction::TransactionTrait;
use crate::application::user_profile::service::UserProfileService;
use crate::environment::Environment;
use crate::infrastructure::GrpcAuthConnector;
use crate::infrastructure::postgres::PostgresTransactionManager;
use crate::infrastructure::postgres::repositories::profile_repository::ProfileRepository;
use crate::infrastructure::postgres::repositories::user_repository::UserRepository;
use crate::infrastructure::secure_hasher::Argon2Hasher;

pub struct ServerState<T> {
    pub user_service: UserProfileService<T>,
    pub profile_service: ProfileService<T>,

    pub domain: String,
}

impl<T: TransactionTrait> ServerState<T> {
    pub fn new(user_service: UserProfileService<T>, profile_service: ProfileService<T>, domain: String) -> Self {
        Self {
            user_service,
            profile_service,
            domain,
        }
    }
}

pub async fn create_state(env: &Environment) -> Result<ServerState<impl TransactionTrait>, anyhow::Error> {
    info!("Connecting to database...");
    let database_url = env.database_url.clone();

    let db_pool_future = task::spawn(async move {
        let time = Instant::now();
        Pool::<Postgres>::connect(&database_url).await
            .map(|pool| {
                info!("Connected to database in {}ms...", time.elapsed().as_millis());
                pool
            })
    });

    let auth_host = env.auth_host.clone();
    let auth_port = env.auth_port;
    let auth_connector_future = task::spawn(async move {
        let time = Instant::now();
        GrpcAuthConnector::connect(auth_host, auth_port)
            .await
            .map(|connector| {
                info!("Connected to auth microservice in {}ms...", time.elapsed().as_millis());
                connector
            })
    });

    let domain = Url::parse(&env.origin)?.host_str().unwrap().to_string();
    info!("Domain parsed from origin: {}", domain);

    info!("Waiting for connections...");
    let db_pool = db_pool_future.await??;
    let auth_connector = auth_connector_future.await??;

    // Initialize repositories
    let transaction_starter = PostgresTransactionManager::new(db_pool.clone());
    let user_repository = UserRepository::new(db_pool.clone());
    let profile_repository = ProfileRepository::new(db_pool.clone());

    // Initialize utilities
    let secure_hasher = Argon2Hasher;

    // Initialize services
    let user_profile_service = UserProfileService::new(
        Box::new(transaction_starter.clone()), Box::new(user_repository.clone()),
        Box::new(profile_repository.clone()),
        Box::new(secure_hasher),
        Box::new(auth_connector));

    let profile_service = ProfileService::new(Box::new(profile_repository.clone()));

    // Resulting state
    Ok(ServerState::new(user_profile_service, profile_service, domain))
}