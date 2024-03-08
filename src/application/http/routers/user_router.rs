pub use user_router::user_router;

mod user_router {
    use std::sync::Arc;

    use axum::Router;
    use axum::routing::post;

    use crate::application::transaction::TransactionTrait;
    use crate::application::user_profile::routes::{sign_in, sign_up};
    use crate::infrastructure::http::state::ServerState;

    pub fn user_router<T: TransactionTrait>() -> Router<Arc<ServerState<T>>> {
        Router::new()
            .route("/users/signup", post(sign_up))
            .route("/users/signin", post(sign_in))
    }
}