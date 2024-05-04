use opentelemetry::metrics;
use opentelemetry::runtime;
use opentelemetry::sdk::export::metrics::aggregation::cumulative_temporality_selector;
use opentelemetry::sdk::metrics::{controllers::BasicController, selectors};
use opentelemetry_otlp::WithExportConfig;

pub(crate) fn opentelemetry_metrics() -> metrics::Result<BasicController> {
    opentelemetry_otlp::new_pipeline()
        .metrics(
            selectors::simple::inexpensive(),
            cumulative_temporality_selector(),
            runtime::Tokio,
        )
        .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
        .build()
}
