use std::sync::Arc;

use axum::{middleware, Router};
use axum::routing::get;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

use crate::application::transaction::TransactionTrait;
use crate::infrastructure::http::middleware::correlation_id_layer::correlation_id_extension;
use crate::infrastructure::http::middleware::session_layer::session_extension;
use crate::infrastructure::http::middleware::tracing_layer::create_tracing_layer;
use crate::infrastructure::http::misc_routes::healthcheck;
use crate::infrastructure::http::state::ServerState;
use crate::infrastructure::profile::router::profile_router;
use crate::infrastructure::user::router::user_router;

pub fn create_router<T: TransactionTrait>(server_state: Arc<ServerState<T>>, cors: CorsLayer) -> Router {
    Router::new()
        .merge(profile_router())
        .merge(user_router())

        .route("/healthcheck", get(healthcheck))


        .layer(middleware::from_fn(session_extension))
        .layer(CookieManagerLayer::new())
        .layer(cors)
        .with_state(server_state)
        .layer(create_tracing_layer())
        .layer(middleware::from_fn(correlation_id_extension))
}