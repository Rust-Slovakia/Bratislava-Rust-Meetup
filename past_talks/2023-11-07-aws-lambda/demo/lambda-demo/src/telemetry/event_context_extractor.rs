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

use lambda_runtime::{self, Context};
use opentelemetry::propagation::Extractor;

/// The `x-amzn-trace-id` is an unique identifier propagated over the AWS components to troubleshoot the issues.
pub const TRACE_ID_HEADER: &str = "x-amzn-trace-id";

/// Extractor implementation for returning the TRACE_ID_HEADER field value from an event context.
#[derive(Debug)]
pub struct EventContextExtractor<'a>(pub &'a Context);

impl<'a> Extractor for EventContextExtractor<'a> {
    /// Get a value for the TRACE_ID_HEADER from the lambda event context. If the key is not TRACE_ID_HEADER, returns None.
    fn get(&self, key: &str) -> Option<&str> {
        if let Some(xray_trace_id) = self.0.xray_trace_id.as_ref() {
            if let TRACE_ID_HEADER = key.to_lowercase().as_str() {
                tracing::info!("Extract from context {}={}", TRACE_ID_HEADER, xray_trace_id);
                Some(xray_trace_id.as_str())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Collect all the keys presented by Extractor.
    fn keys(&self) -> Vec<&str> {
        vec![TRACE_ID_HEADER]
    }
}
