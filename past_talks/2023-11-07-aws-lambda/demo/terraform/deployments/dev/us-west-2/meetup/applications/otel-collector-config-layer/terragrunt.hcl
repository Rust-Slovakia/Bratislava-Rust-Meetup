terraform {
  source = "${get_terragrunt_dir()}/../../../../../../..//terraform/modules/otel-collector-config-layer"
}

include {
  path = find_in_parent_folders()
}

inputs = {
  name = "service_gaas_otel_collector_config_layer"
}