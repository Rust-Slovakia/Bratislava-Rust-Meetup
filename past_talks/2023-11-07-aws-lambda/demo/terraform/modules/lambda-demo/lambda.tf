
resource "aws_lambda_function" "lambda" {
  function_name = local.full_name
  description   = "Meetup Lambda DEMO"
  handler       = "main"
  runtime       = "provided.al2"
  filename      = "../../../lambda-demo/target/lambda/lambda-demo/bootstrap.zip"

  role = aws_iam_role.this.arn

  source_code_hash = filebase64sha256("../../../lambda-demo/target/lambda/lambda-demo/bootstrap.zip")

  timeout = 10

  ephemeral_storage {
    size = 512
  }

  environment {
    variables = {
      RUST_LOG                            = var.rust_log
      OPENTELEMETRY_COLLECTOR_CONFIG_FILE = "/opt/${var.otel_collector_configuration_yaml}"
    }
  }

  tracing_config {
    mode = "Active"
  }

  layers = compact([
    var.otel_collector_config_layer_arn,
    var.otel_collector_layer_arn,
  ])

  depends_on = [
    aws_cloudwatch_log_group.this,
  ]
}
