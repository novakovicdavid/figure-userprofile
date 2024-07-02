use std::sync::Arc;

use crate::environment::Environment;
use crate::infrastructure::http::server::start_server;
use crate::infrastructure::logging::init_logging;
use crate::state::create_state;

mod domain;
mod infrastructure;
mod application;
mod environment;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    init_logging("INFO", "WARN").unwrap();

    let environment = Environment::new()?;

    let state = Arc::new(create_state(&environment).await?);

    state.migration_runner.run().await?;

    start_server(&environment, state.clone()).await
}
