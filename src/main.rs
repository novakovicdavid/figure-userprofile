use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use axum::{middleware, Router};
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use http::header::{ACCEPT, CONTENT_TYPE};
use http::Method;
use sqlx::{Pool, Postgres};
use tokio::task;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tracing::info;
use url::Url;
use crate::application::profile::routes::{get_profile, get_total_profiles_count, update_profile};
use crate::application::profile::service::ProfileService;
use crate::application::user::routes::{sign_in, sign_out, sign_up};
use crate::application::user::service::UserService;
use crate::context::{Context, ContextTrait, RepositoryContext, ServiceContext};
use crate::environment::Environment;
use crate::infrastructure::middleware::correlation_id_layer::correlation_id_extension;
use crate::infrastructure::middleware::session_layer::session_extension;
use crate::infrastructure::middleware::tracing_layer::create_tracing_layer;
use crate::infrastructure::misc_routes::healthcheck;
use crate::infrastructure::profile::repository::ProfileRepository;
use crate::infrastructure::secure_rand_generator::ChaCha20;
use crate::infrastructure::transaction::PostgresTransactionManager;
use crate::infrastructure::user::repository::UserRepository;

mod domain;
mod infrastructure;
mod server_errors;
mod application;
mod context;
mod environment;

pub struct ServerState<C: ContextTrait> {
    context: C,
    domain: String,
}

impl<C: ContextTrait> ServerState<C> {
    pub fn new(context: C, domain: String) -> Self {
        Self {
            context,
            domain,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let time_to_start = Instant::now();

    let env = Environment::new()?;

    info!("Connecting to database...");
    let db_pool_future = task::spawn(async move {
        let time = Instant::now();
        Pool::<Postgres>::connect(&env.database_url).await
            .map(|pool| {
                info!("Connected to database in {}ms...", time.elapsed().as_millis());
                pool
            })
    });


    info!("Setting up CORS...");
    let cors = create_app_cors([env.origin.parse()?]);
    info!("Allowed origin (CORS): {}", env.origin);

    let domain = Url::parse(&env.origin)?.host_str().unwrap().to_string();
    info!("Domain parsed from origin: {}", domain);

    info!("Waiting for stores...");
    let db_pool = db_pool_future.await??;

    info!("Creating state...");
    let server_state = create_state(db_pool, domain);

    info!("Setting up routes and layers...");
    let app = create_app(server_state, cors);

    let server_port = env.server_port;
    let addr = SocketAddr::from(([0, 0, 0, 0], server_port));

    let socket = tokio::net::TcpListener::bind(addr).await?;

    info!("Starting Axum...");
    let axum_server = axum::serve(socket, app);

    info!("Server is up at port {server_port}");
    info!("Ready to serve in {}ms", time_to_start.elapsed().as_millis());

    axum_server.await?;
    Ok(())
}

fn create_app<C: ContextTrait + 'static>(server_state: Arc<ServerState<C>>, cors: CorsLayer) -> Router {
    Router::new()
        .route("/profile/update", post(update_profile))
        // Disable the default limit
        .layer(DefaultBodyLimit::disable())
        // Set a different limit
        .layer(RequestBodyLimitLayer::new(5 * 1000000))

        .route("/healthcheck", get(healthcheck))
        .route("/users/signup", post(sign_up))
        .route("/users/signin", post(sign_in))
        .route("/profiles/:id", get(get_profile))
        .route("/profiles/count", get(get_total_profiles_count))

        .layer(middleware::from_fn(session_extension))
        .layer(CookieManagerLayer::new())
        .layer(cors)
        .with_state(server_state)
        .layer(create_tracing_layer())
        .layer(middleware::from_fn(correlation_id_extension))
}

fn create_state(db_pool: Pool<Postgres>, domain: String) -> Arc<ServerState<impl ContextTrait>> {
    // Initialize repositories
    let transaction_starter = PostgresTransactionManager::new(db_pool.clone());
    let user_repository = UserRepository::new(db_pool.clone());
    let profile_repository = ProfileRepository::new(db_pool.clone());

    // Initialize utilities
    let secure_random_generator = ChaCha20::new();

    // Initialize services
    let user_service = UserService::new(
        Box::new(transaction_starter.clone()), Box::new(user_repository.clone()),
        Box::new(profile_repository.clone()),
        Box::new(secure_random_generator));

    let profile_service = ProfileService::new(Box::new(profile_repository.clone()));

    // Create service and repository contexts
    let repository_context = RepositoryContext::new(user_repository, profile_repository);
    let service_context = ServiceContext::new(user_service, profile_service);

    // Combine contexts
    let context = Context::new(service_context, repository_context);

    // Resulting state
    Arc::new(ServerState::new(context, domain))
}

fn create_app_cors<T: Into<AllowOrigin>>(origins: T) -> CorsLayer {
    CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([ACCEPT, CONTENT_TYPE])
        .allow_origin(origins)
}
