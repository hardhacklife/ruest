//! Structured logging for RustForge (tracing).

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize the default RustForge tracing subscriber.
pub fn init() {
    init_with_filter("info");
}

/// Initialize tracing with a custom filter directive.
pub fn init_with_filter(filter: &str) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
