use std::time::Instant;

use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use figure_lib::rdbs::postgres::tokio_postgres::outbox::TokioPostgresOutbox;
use figure_lib::rdbs::transaction::postgres_transaction::{TransactionBackend, TransactionManager};
use tokio::task;
use tokio_postgres::NoTls;
use tracing::log::info;
use url::Url;

use crate::application::migration_runner_trait::MigrationRunner;
use crate::application::services::profile_service::ProfileService;
use crate::application::services::user_service::UserProfileService;
use crate::environment::Environment;
use crate::infrastructure::database::repositories::profile_repository::PostgresProfileRepository;
use crate::infrastructure::database::repositories::user_repository::PostgresTokioUserRepository;
use crate::infrastructure::database::TokioPostgresMigrationRunner;
use crate::infrastructure::GrpcAuthConnector;

pub struct ServerState {
    pub migration_runner: Box<dyn MigrationRunner>,
    pub user_service: UserProfileService,
    pub profile_service: ProfileService,

    pub domain: String,
}

impl ServerState {
    pub fn new(migration_runner: Box<dyn MigrationRunner>,
               user_service: UserProfileService,
               profile_service: ProfileService,
               domain: String) -> Self {
        Self {
            migration_runner,
            user_service,
            profile_service,
            domain,
        }
    }
}

pub async fn create_state(env: &Environment) -> Result<ServerState, anyhow::Error> {
    info!("Connecting to database...");

    let database_url = env.database_url.clone();
    let db_pool_future = task::spawn(async move {
        let mut cfg = Config::new();
        cfg.url = Some(database_url);
        cfg.dbname = Some("postgres".to_string());
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        // TODO tls (important!!!)
        cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
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
    let db_pool = db_pool_future.await?;
    let auth_connector = auth_connector_future.await??;

    // Initialize repositories
    let migration_runner = Box::new(TokioPostgresMigrationRunner::new(db_pool.clone()));
    let transaction_starter = TransactionManager::new(TransactionBackend::PostgresTokio(db_pool.clone()));
    let user_repository = PostgresTokioUserRepository::new(db_pool.clone());
    let profile_repository = PostgresProfileRepository::new(db_pool);
    let outbox_repository = TokioPostgresOutbox::new();

    // Initialize services
    let user_profile_service = UserProfileService::new(
        transaction_starter.clone(), Box::new(user_repository),
        Box::new(profile_repository.clone()),
        Box::new(outbox_repository),
        Box::new(auth_connector));

    let profile_service = ProfileService::new(Box::new(profile_repository));

    // Resulting state
    Ok(ServerState::new(migration_runner, user_profile_service, profile_service, domain))
}