use tracing::Subscriber;
use tracing_subscriber::{EnvFilter, fmt, Layer};
use tracing_subscriber::filter::Filtered;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_logging(application_level: &str, library_level: &str) -> Result<(), anyhow::Error> {
    let registry = tracing_subscriber::registry();

    let console_output = add_filter_to_layer(fmt::layer(), application_level, library_level)?;

    registry
        .with(console_output)
        .init();

    Ok(())
}

fn add_filter_to_layer<S: Subscriber, L: Layer<S>>(layer: L, application_level: &str, library_level: &str) -> Result<Filtered<L, EnvFilter, S>, anyhow::Error> {
    let crate_name = env!("CARGO_PKG_NAME").replace("-", "_");

    Ok(layer
        .with_filter(EnvFilter::default()
            .add_directive(library_level.parse()?)
            .add_directive(format!("figure_lib={}", application_level).parse()?)
            .add_directive(format!("{crate_name}={}", application_level).parse()?)))
}