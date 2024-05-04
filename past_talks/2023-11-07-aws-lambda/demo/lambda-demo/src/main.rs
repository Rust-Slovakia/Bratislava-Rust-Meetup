mod telemetry;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use opentelemetry::global::{self, shutdown_tracer_provider};
use serde::{Deserialize, Serialize};
use telemetry::{
    telemetry_lambda::{tracer_init, TRACER_PROVIDER},
    EventContextExtractor,
};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Debug, Deserialize)]
struct Request {
    command: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Debug, Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
#[tracing::instrument(ret, err)]
async fn function_handler(lambda_event: LambdaEvent<Request>) -> Result<Response, Error> {
    let (payload, context) = lambda_event.into_parts();

    // initialize the current span context with the values from an event context
    let parent_context = global::get_text_map_propagator(|propagator| {
        propagator.extract(&EventContextExtractor(&context))
    });
    tracing::Span::current().set_parent(parent_context);

    // Extract some useful info from the request
    let command = payload.command;

    // Prepare the response
    let resp = Response {
        req_id: context.request_id,
        msg: format!("Command {}.", command),
    };

    if let Some(tracer_provider) = TRACER_PROVIDER.get() {
        tracer_provider.force_flush();
    }

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

pub fn get_app_version_with_revision() -> String {
    let version = env!("CARGO_PKG_VERSION");
    match std::env::var("APP_REVISION") {
        Ok(env) => format!("{}-{}", version, env),
        _ => version.to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracer_init(env!("CARGO_PKG_NAME"), &get_app_version_with_revision())?;

    let result = run(service_fn(function_handler)).await;

    shutdown_tracer_provider();

    result
}
