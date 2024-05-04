variable "name" {
  type    = string
  default = "lambda-demo"
}

variable "aws_region" {
  type        = string
  description = "AWS region"
}

variable "environment" {
  type = string
}

variable "organization" {
  type = string
}

variable "rust_log" {
  type        = string
  description = "Log level, should be info or debug, defaults to info"
  default     = "info"
}

variable "otel_collector_config_layer_arn" {
  type        = string
  description = "The ARN of the Lambda Layer containing the OTEL Collector Configuration"
}

variable "otel_collector_layer_arn" {
  type        = string
  description = "The ARN of the Lambda Layer running the OTEL Collector"
}

variable "otel_collector_configuration_yaml" {
  type        = string
  description = "OTEL collector configuration file"
  default     = "collector.yaml"
}
