terraform {
  source = "${get_terragrunt_dir()}/../../../../../../..//terraform/modules/lambda-demo"
}

include {
  path = find_in_parent_folders()
}

dependency "otel-collector-config-layer" {
  config_path = "../otel-collector-config-layer"
}

inputs = {
  name = "lambda-demo"

  otel_collector_config_layer_arn = dependency.otel-collector-config-layer.outputs.otel_collector_config_layer_arn,

  otel_collector_layer_arn = dependency.otel-collector-config-layer.outputs.otel_collector_layer_arn,

  otel_collector_configuration_yaml = dependency.otel-collector-config-layer.outputs.otel_collector_configuration_yaml
} 