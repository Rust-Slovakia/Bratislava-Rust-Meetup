resource "aws_lambda_layer_version" "otel_config" {
  depends_on = [
    data.archive_file.default
  ]

  layer_name = local.full_name

  filename         = var.collector_configuration_zip
  source_code_hash = data.archive_file.default.output_base64sha256
}

data "archive_file" "default" {
  type        = "zip"
  source_file = var.collector_configuration_yaml
  output_path = var.collector_configuration_zip
}
