use opentelemetry::{
    global,
    sdk::{
        trace::{self, Sampler, Tracer},
        Resource,
    },
    trace::TraceError,
    KeyValue,
};

use opentelemetry_aws::XrayPropagator;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_semantic_conventions as semcov;

pub(crate) fn opentelemetry_tracer(app_name: &str, version: &str) -> Result<Tracer, TraceError> {
    global::set_text_map_propagator(XrayPropagator::new());

    let environment = match std::env::var("DEPLOYMENT_ENVIRONMENT") {
        Ok(env) => env,
        _ => "dev".to_string(),
    };

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
        .with_trace_config(
            trace::config()
                .with_resource(Resource::new(vec![
                    KeyValue::new(
                        semcov::resource::SERVICE_NAME.to_string(),
                        app_name.to_string(),
                    ),
                    KeyValue::new(
                        semcov::resource::SERVICE_VERSION.to_string(),
                        version.to_string(),
                    ),
                    KeyValue::new(
                        semcov::resource::SERVICE_INSTANCE_ID.to_string(),
                        gethostname::gethostname().to_string_lossy().to_string(),
                    ),
                    KeyValue::new(semcov::resource::SERVICE_NAMESPACE.to_string(), environment),
                ]))
                .with_sampler(Sampler::AlwaysOn)
                // Needed in order to convert the trace IDs into an Xray-compatible format
                .with_id_generator(trace::XrayIdGenerator::default()),
        )
        .install_batch(opentelemetry::runtime::Tokio)
}
