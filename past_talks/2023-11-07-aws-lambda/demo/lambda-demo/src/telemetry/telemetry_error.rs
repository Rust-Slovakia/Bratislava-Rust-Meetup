use opentelemetry::{metrics::MetricsError, trace::TraceError};
use thiserror::Error;

#[derive(Error)]
/// Represents the error happened while handling the telemetry
pub enum TelemetryError {
    /// Returned when unexpected error happened.
    #[error("Unexpected error {0}")]
    Unexpected(String),

    /// Errors returned by trace API
    #[error(transparent)]
    TraceError(#[from] TraceError),

    /// Errors returned by metrics API
    #[error(transparent)]
    MetricsError(#[from] MetricsError),
}

impl std::fmt::Debug for TelemetryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

/// Formats the chained errors
pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
