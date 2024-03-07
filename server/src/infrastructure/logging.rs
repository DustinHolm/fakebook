use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::prelude::*;

pub fn init() -> WorkerGuard {
    let (non_blocking_writer, guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking_writer))
        .init();

    guard
}
