pub use user_router::user_router;

mod user_router {
    use std::sync::Arc;

    use axum::Router;
    use axum::routing::post;
    use figure_lib::rdbs::transaction::TransactionTrait;

    use crate::application::user_profile::routes::{sign_in, sign_up};
    use crate::state::ServerState;

    pub fn user_router<T: TransactionTrait>() -> Router<Arc<ServerState<T>>> {
        Router::new()
            .route("/user/signup", post(sign_up))
            .route("/user/signin", post(sign_in))
    }
}