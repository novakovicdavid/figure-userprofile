use std::sync::Arc;

use axum::{middleware, Router};
use axum::routing::get;
use figure_lib::middleware::correlation_id::CorrelationLayer;
use figure_lib::middleware::tracing::http_tracing_layer;
use http::header::{ACCEPT, CONTENT_TYPE};
use http::Method;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::application::profile::http_router::profile_router;
use crate::application::user_profile::http_router::user_router;
use crate::infrastructure::http::middleware::session_layer::session_extension;
use crate::infrastructure::http::misc_routes::healthcheck;
use crate::state::ServerState;

pub fn create_router(server_state: Arc<ServerState>, cors: &String) -> Result<Router, anyhow::Error> {
    let cors_layer = create_cors_layer([cors.parse()?]);

    let router = Router::new()
        .merge(profile_router())
        .merge(user_router())

        .route("/healthcheck", get(healthcheck))


        .layer(middleware::from_fn(session_extension))
        .layer(CookieManagerLayer::new())
        .layer(cors_layer)
        .with_state(server_state)
        .layer(http_tracing_layer())
        .layer(CorrelationLayer);

    Ok(router)
}

fn create_cors_layer<T: Into<AllowOrigin>>(origins: T) -> CorsLayer {
    CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([ACCEPT, CONTENT_TYPE])
        .allow_origin(origins)
}