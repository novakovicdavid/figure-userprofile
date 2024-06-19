pub use user_router::user_router;

mod user_router {
    use std::sync::Arc;

    use axum::Router;
    use axum::routing::post;

    use crate::application::user_profile::http_routes::{reset_password, sign_in, sign_up};
    use crate::state::ServerState;

    pub fn user_router() -> Router<Arc<ServerState>> {
        Router::new()
            .route("/user/reset-password", post(reset_password))
            .route("/user/signup", post(sign_up))
            .route("/user/signin", post(sign_in))
    }
}