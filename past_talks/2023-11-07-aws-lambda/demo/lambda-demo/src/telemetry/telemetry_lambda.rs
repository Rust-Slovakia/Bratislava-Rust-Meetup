use super::metrics::opentelemetry_metrics;
use super::tracer::opentelemetry_tracer;
use super::TelemetryError;
use once_cell::sync::OnceCell;
use opentelemetry::sdk::trace::TracerProvider;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::{fmt, prelude::*, Registry};

/// TracerProvider global resource
pub static TRACER_PROVIDER: OnceCell<TracerProvider> = OnceCell::new();

/// Initialize the tracing layer
/// This method panics if a global default subscriber has already been set.
pub fn tracer_init(app_name: &str, version: &str) -> Result<(), TelemetryError> {
    let tracer = opentelemetry_tracer(app_name, version)?;
    let tracer_provider = tracer
        .provider()
        .ok_or_else(|| TelemetryError::Unexpected("Failed to get TracerProvider".to_string()))?;
    TRACER_PROVIDER.set(tracer_provider).unwrap();

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let _opentelemetry_metrics = opentelemetry_metrics()?;

    Registry::default()
        .with(
            EnvFilter::builder()
                .with_default_directive("info".parse().unwrap())
                .from_env_lossy(),
        )
        .with(opentelemetry)
        .with(fmt::layer().without_time().json())
        .init();
    Ok(())
}
