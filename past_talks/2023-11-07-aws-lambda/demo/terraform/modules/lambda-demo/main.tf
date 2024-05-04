terraform {
  required_version = "1.2.8"

  backend "s3" {}

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "5.24.0"
    }
  }
}

provider "aws" {
  profile = "${var.organization}-${var.environment}"
  region  = var.aws_region

  default_tags {
    tags = {
      ManagedByTerraform = true
      TerraformLocation  = "https://github.com/Rust-Slovakia/Bratislava-Rust-Meetup"
      Environment        = var.environment
      Service            = local.full_name
    }
  }
}
