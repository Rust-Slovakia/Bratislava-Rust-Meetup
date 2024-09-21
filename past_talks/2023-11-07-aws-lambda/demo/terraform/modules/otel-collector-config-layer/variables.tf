variable "name" {
  type    = string
  default = "otel_collector_config"
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

variable "collector_configuration_yaml" {
  type        = string
  description = "OTEL collector configuration file"
  default     = "collector.yaml"
}

variable "collector_configuration_zip" {
  type        = string
  description = "ZIP-ed OTEL collector configuration file"
  default     = "collector.zip"
}
