use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use tracing::log::info;

use crate::application::environment::Environment;
use crate::application::routes::ConnectionInfo;
use crate::application::state::ServerState;
use crate::infrastructure::http::router::create_router;

pub async fn start_http_server(env: &Environment, state: Arc<ServerState>) -> Result<(), anyhow::Error> {
    let time_to_start = Instant::now();

    info!("Allowed origin (CORS): {}", env.origin);

    info!("Setting up routes and layers...");
    let router = create_router(state, &env.origin)?;

    let server_port = env.server_port;
    let addr = SocketAddr::from(([0, 0, 0, 0], server_port));

    let socket = tokio::net::TcpListener::bind(addr).await?;

    info!("Starting Axum...");
    let axum_server = axum::serve(socket, router.into_make_service_with_connect_info::<ConnectionInfo>());

    info!("Server is up at port {server_port}");
    info!("Ready to serve in {}ms", time_to_start.elapsed().as_millis());

    axum_server.await?;
    Ok(())
}