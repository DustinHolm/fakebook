use std::collections::HashMap;

use hyper::{HeaderMap, Request};
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tower_http::trace::MakeSpan;
use tracing::{span, Level, Span};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::prelude::*;

use super::errors::InfrastructureError;

pub fn init() -> Result<WorkerGuard, InfrastructureError> {
    #[cfg(feature = "otel")]
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .map_err(|e| InfrastructureError::Logging(Box::new(e)))?;

    #[cfg(feature = "otel")]
    let tracer = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
        .build()
        .tracer("fakebook-server");

    let (non_blocking_writer, guard) = tracing_appender::non_blocking(std::io::stdout());

    let registry = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking_writer));

    #[cfg(feature = "otel")]
    registry
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
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

pub fn current_span_as_headers() -> HeaderMap {
    use opentelemetry::propagation::TextMapPropagator;
    use tracing_opentelemetry::OpenTelemetrySpanExt;

    let propagator = TraceContextPropagator::new();
    let mut header_map = HashMap::new();
    let trace_context = Span::current().context();

    propagator.inject_context(&trace_context, &mut header_map);

    HeaderMap::try_from(&header_map).unwrap_or_default()
}
