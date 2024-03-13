pub use profile_router::profile_router;

mod profile_router {
    use std::sync::Arc;

    use axum::Router;
    use axum::routing::{get, post};
    use figure_lib::rdbs::transaction::TransactionTrait;
    use tower_http::limit::RequestBodyLimitLayer;

    use crate::application::profile::routes::{get_profile, get_total_profiles_count, update_profile};
    use crate::state::ServerState;

    pub fn profile_router<T: TransactionTrait>() -> Router<Arc<ServerState<T>>> {
        Router::new()
            .route("/profile/update", post(update_profile)
                // Set a different limit
                .layer(RequestBodyLimitLayer::new(5 * 1_000_000)))

            .route("/profiles/:id", get(get_profile))
            .route("/profiles/count", get(get_total_profiles_count))
    }
}