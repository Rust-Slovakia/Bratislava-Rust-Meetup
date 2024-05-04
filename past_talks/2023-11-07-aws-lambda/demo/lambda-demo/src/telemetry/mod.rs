#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

//! Telemetry module implements the tracing layer.
//! We follow the [OpenTelemetry](https://opentelemetry.io/docs/) standard
//! to export telemetry data such as  traces, metrics and logs.

mod metrics;
mod tracer;

mod event_context_extractor;
pub use event_context_extractor::*;

mod telemetry_error;
pub use telemetry_error::*;

/// lambda function telemetry
pub mod telemetry_lambda;
