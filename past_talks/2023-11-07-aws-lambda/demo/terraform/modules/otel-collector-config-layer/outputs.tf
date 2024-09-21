output "otel_collector_config_layer_arn" {
  description = "The ARN of the Lambda Layer containing the OTEL Collector Configuration"
  value       = aws_lambda_layer_version.otel_config.arn
}

output "otel_collector_layer_arn" {
  description = "The ARN of the Lambda Layer running the OTEL Collector"
  value       = local.architecture_to_arns_mapping["x86_64"][var.aws_region]
}

output "otel_collector_configuration_yaml" {
  description = "The OTEL Collector Configuration file name"
  value       = var.collector_configuration_yaml
}
