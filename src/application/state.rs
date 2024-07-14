use std::sync::Arc;
use std::time::Instant;

use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use figure_lib::queue::integration::domain_event_dispatcher::DomainEventDispatcher;
use figure_lib::rdbs::outbox_repository::Outbox;
use figure_lib::rdbs::postgres::tokio_postgres::outbox::TokioPostgresOutbox;
use figure_lib::rdbs::transaction::postgres_transaction::{TransactionBackend, TransactionManager};
use tokio::task;
use tokio_postgres::NoTls;
use tracing::log::info;
use url::Url;

use crate::application::connectors::auth_connector::AuthConnector;
use crate::application::domain_event_dispatcher::{DomainEvent, DomainEventDiscriminants};
use crate::application::domain_event_handlers::user_created::password_reset_requested;
use crate::application::environment::Environment;
use crate::application::migration_runner_trait::MigrationRunner;
use crate::application::repository_traits::read::profile_repository::ProfileRepository;
use crate::application::repository_traits::read::user_repository::UserRepository;
use crate::application::services::profile_service::ProfileService;
use crate::application::services::user_service::UserProfileService;
use crate::infrastructure::database::repositories::profile_repository::PostgresProfileRepository;
use crate::infrastructure::database::repositories::user_repository::TokioPostgresUserRepository;
use crate::infrastructure::database::TokioPostgresMigrationRunner;
use crate::infrastructure::GrpcAuthConnector;

pub struct ServerState {
    pub migration_runner: Box<dyn MigrationRunner>,
    pub domain_dispatcher: Arc<DomainEventDispatcher<DomainEventDiscriminants, DomainEvent, Arc<DomainEventHandlerState>>>,
    pub user_service: UserProfileService,
    pub profile_service: ProfileService,

    pub domain: String,
}

impl ServerState {
    pub fn new(migration_runner: Box<dyn MigrationRunner>,
               domain_dispatcher: Arc<DomainEventDispatcher<
                   DomainEventDiscriminants,
                   DomainEvent,
                   Arc<DomainEventHandlerState>>>,
               user_service: UserProfileService,
               profile_service: ProfileService,
               domain: String)
               -> Self {
        Self { migration_runner, domain_dispatcher, user_service, profile_service, domain }
    }
}

pub struct DomainEventHandlerState {
    pub transaction_manager: TransactionManager,
    pub user_repository: Box<dyn UserRepository>,
    pub profile_repository: Box<dyn ProfileRepository>,
    pub outbox_repository: Box<dyn Outbox>,
    pub auth_connector: Box<dyn AuthConnector>,
}

pub async fn create_state(env: &Environment) -> Result<Arc<ServerState>, anyhow::Error> {
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
    let user_repository = TokioPostgresUserRepository::new(db_pool.clone());
    let profile_repository = PostgresProfileRepository::new(db_pool);
    let outbox_repository = TokioPostgresOutbox::new();

    let domain_event_dispatcher: DomainEventDispatcher<DomainEventDiscriminants, DomainEvent, _> =
        DomainEventDispatcher::new(Arc::new(DomainEventHandlerState {
            transaction_manager: transaction_starter.clone(),
            user_repository: Box::new(user_repository.clone()),
            profile_repository: Box::new(profile_repository.clone()),
            outbox_repository: Box::new(outbox_repository.clone()),
            auth_connector: Box::new(auth_connector.clone()),
        }))
            .register(password_reset_requested);

    let domain_event_dispatcher = Arc::new(domain_event_dispatcher);

    // Initialize services
    let user_service = UserProfileService::new(
        transaction_starter.clone(), domain_event_dispatcher.clone(),
        Box::new(user_repository),
        Box::new(profile_repository.clone()),
        Box::new(outbox_repository),
        Box::new(auth_connector));

    let profile_service = ProfileService::new(Box::new(profile_repository));

    // Resulting state
    Ok(Arc::new(ServerState::new(migration_runner, domain_event_dispatcher, user_service, profile_service, domain)))
}