use crate::prelude::Res;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn setup() -> Res<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(())
}
