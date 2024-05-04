locals {
  # Load common variables
  environment_vars = read_terragrunt_config(find_in_parent_folders("environment.hcl"))
  region_vars = read_terragrunt_config(find_in_parent_folders("region.hcl"))
  organization_vars = read_terragrunt_config(find_in_parent_folders("organization.hcl"))

  environment = local.environment_vars.locals.environment
  aws_region = local.region_vars.locals.aws_region
  organization = local.organization_vars.locals.organization

  # Building configuration values for a child module's backend terraform state
  remote_state_s3_key = "env:/${join(".", compact([local.organization, local.environment]))}/terraform/${path_relative_to_include()}"
  remote_state_bucket = join("-", [local.organization, local.environment, "terraform"])
  remote_state_profile = join("-", [local.organization, local.environment])
  remote_state_dynamodb_table = join("-", [local.organization, local.environment, "terraform", "locks"])
}

# Configure Terragrunt to store state in S3 bucket
remote_state {
  backend = "s3"
  config = {
    encrypt        = true
    bucket         = local.remote_state_bucket
    region         = local.aws_region
    key            = local.remote_state_s3_key
    profile        = local.remote_state_profile
    dynamodb_table = local.remote_state_dynamodb_table
  }
}

# inputs is a map of values to be passed to modules that inherit this config
inputs = merge(
  local.region_vars.locals,
  local.environment_vars.locals,
  local.organization_vars.locals,
)
