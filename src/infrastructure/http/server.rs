use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use figure_lib::rdbs::transaction::TransactionTrait;
use tracing::log::info;

use crate::environment::Environment;
use crate::infrastructure::http::router::create_router;
use crate::state::ServerState;

pub async fn start_server<T: TransactionTrait>(env: &Environment, state: Arc<ServerState<T>>) -> Result<(), anyhow::Error> {
    let time_to_start = Instant::now();

    info!("Allowed origin (CORS): {}", env.origin);

    info!("Setting up routes and layers...");
    let router = create_router(state, &env.origin)?;

    let server_port = env.server_port;
    let addr = SocketAddr::from(([0, 0, 0, 0], server_port));

    let socket = tokio::net::TcpListener::bind(addr).await?;

    info!("Starting Axum...");
    let axum_server = axum::serve(socket, router);

    info!("Server is up at port {server_port}");
    info!("Ready to serve in {}ms", time_to_start.elapsed().as_millis());

    axum_server.await?;
    Ok(())
}