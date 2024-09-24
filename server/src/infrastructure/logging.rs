use hyper::Request;
use opentelemetry::{global, trace::TracerProvider};
#[cfg(feature = "otel")]
use opentelemetry_sdk::runtime;
use tower_http::trace::MakeSpan;
use tracing::{span, Level, Span};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::prelude::*;

use super::errors::InfrastructureError;

pub fn init() -> Result<WorkerGuard, InfrastructureError> {
    #[cfg(feature = "otel")]
    let tracing = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(runtime::Tokio)
        .map_err(|e| InfrastructureError::Logging(Box::new(e)))?;

    let (non_blocking_writer, guard) = tracing_appender::non_blocking(std::io::stdout());

    let registry = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking_writer));

    #[cfg(feature = "otel")]
    registry
        .with(tracing_opentelemetry::layer().with_tracer(tracing.tracer("fakebook-server")))
        .init();

    #[cfg(not(feature = "otel"))]
    registry.init();

    Ok(guard)
}

#[derive(Clone)]
pub(super) struct CustomMakeSpan;

impl<B> MakeSpan<B> for CustomMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        span!(
            Level::DEBUG,
            "request",
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version(),
            otel.kind = "server"
        )
    }
}
