use crate::environment::Environment;
use crate::infrastructure::http::server::start_server;
use crate::infrastructure::http::state::create_state;
use crate::infrastructure::logging::init_logging;

mod domain;
mod infrastructure;
mod application;
mod environment;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let environment = Environment::new()?;

    init_logging("INFO", "WARN").unwrap();

    start_server(&environment, create_state(&environment).await?)
        .await
}
