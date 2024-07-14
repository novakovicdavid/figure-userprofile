use application::environment::Environment;
use application::state::create_state;

use crate::infrastructure::http::server::start_http_server;
use crate::infrastructure::logging::init_logging;

mod domain;
mod infrastructure;
mod application;

#[tokio::main]
async fn main() {
    init_logging("INFO", "WARN").unwrap();

    let environment = Environment::new().unwrap();

    let state = create_state(&environment).await.unwrap();

    state.migration_runner.run().await.unwrap();

    start_http_server(&environment, state.clone()).await.unwrap()
}
