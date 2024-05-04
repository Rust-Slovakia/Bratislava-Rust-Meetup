locals {
  full_name = "${var.organization}-${var.environment}-${var.name}"

  architecture_to_arns_mapping = {
    "x86_64" = local.collector_layer_arns_amd64
  }
}
