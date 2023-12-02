use tracing::{info, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[instrument(target = "crate::log")]
pub fn init_logger() {
    let app = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    //TODO: could log to a file as well, or journal for fatal errors
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{app}=debug").into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("{app} v{version} has started");
}
