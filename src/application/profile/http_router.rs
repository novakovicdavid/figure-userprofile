pub use profile_router::profile_router;

mod profile_router {
    use std::sync::Arc;

    use axum::Router;
    use axum::routing::{get, post};
    use tower_http::limit::RequestBodyLimitLayer;

    use crate::application::profile::http_routes::{get_profile, get_total_profiles_count, update_profile};
    use crate::state::ServerState;

    pub fn profile_router() -> Router<Arc<ServerState>> {
        Router::new()
            .route("/profile/update", post(update_profile)
                // Set a different limit
                .layer(RequestBodyLimitLayer::new(5 * 1_000_000)))

            .route("/profiles/:id", get(get_profile))
            .route("/profiles/count", get(get_total_profiles_count))
    }
}