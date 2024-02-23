use std::sync::Arc;

use axum::{middleware, Router};
use axum::routing::get;
use http::header::{ACCEPT, CONTENT_TYPE};
use http::Method;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::application::transaction::TransactionTrait;
use crate::infrastructure::http::middleware::correlation_id_layer::correlation_id_extension;
use crate::infrastructure::http::middleware::session_layer::session_extension;
use crate::infrastructure::http::middleware::tracing_layer::create_tracing_layer;
use crate::infrastructure::http::misc_routes::healthcheck;
use crate::infrastructure::http::state::ServerState;
use crate::infrastructure::profile::router::profile_router;
use crate::infrastructure::user::router::user_router;

pub fn create_router<T: TransactionTrait>(server_state: Arc<ServerState<T>>, cors: &String) -> Result<Router, anyhow::Error> {
    let cors_layer = create_cors_layer([cors.parse()?]);

    let router = Router::new()
        .merge(profile_router())
        .merge(user_router())

        .route("/healthcheck", get(healthcheck))


        .layer(middleware::from_fn(session_extension))
        .layer(CookieManagerLayer::new())
        .layer(cors_layer)
        .with_state(server_state)
        .layer(create_tracing_layer())
        .layer(middleware::from_fn(correlation_id_extension));

    Ok(router)
}

fn create_cors_layer<T: Into<AllowOrigin>>(origins: T) -> CorsLayer {
    CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([ACCEPT, CONTENT_TYPE])
        .allow_origin(origins)
}