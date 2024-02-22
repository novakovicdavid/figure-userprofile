use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use sqlx::{Pool, Postgres};
use tokio::task;
use tracing::info;
use url::Url;

use crate::application::transaction::TransactionTrait;
use crate::create_app_cors;
use crate::environment::Environment;
use crate::infrastructure::http::router::create_router;
use crate::infrastructure::http::state::{create_state, ServerState};
use crate::infrastructure::logging::init_logging;

pub async fn start_server<T: TransactionTrait>(env: &Environment, state: ServerState<T>) -> Result<(), anyhow::Error> {
    let time_to_start = Instant::now();

    let cors = create_app_cors([env.origin.parse()?]);
    info!("Allowed origin (CORS): {}", env.origin);

    info!("Setting up routes and layers...");
    let app = create_router(Arc::new(state), cors);

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