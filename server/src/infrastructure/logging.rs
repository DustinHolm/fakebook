use opentelemetry_sdk::runtime;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::prelude::*;

use super::errors::InfrastructureError;

pub fn init() -> Result<WorkerGuard, InfrastructureError> {
    let tracing = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(runtime::Tokio)
        .map_err(|e| InfrastructureError::Logging(Box::new(e)))?;

    let (non_blocking_writer, guard) = tracing_appender::non_blocking(std::io::stdout());

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking_writer))
        .with(tracing_opentelemetry::layer().with_tracer(tracing))
        .init();

    Ok(guard)
}
