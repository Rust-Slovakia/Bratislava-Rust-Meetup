<!-- BEGIN_TF_DOCS -->
## Requirements

| Name | Version |
|------|---------|
| <a name="requirement_aws"></a> [aws](#requirement\_aws) | 4.31.0 |

## Providers

| Name | Version |
|------|---------|
| <a name="provider_archive"></a> [archive](#provider\_archive) | n/a |
| <a name="provider_aws"></a> [aws](#provider\_aws) | 4.31.0 |

## Modules

No modules.

## Resources

| Name | Type |
|------|------|
| [aws_lambda_layer_version.otel_config](https://registry.terraform.io/providers/hashicorp/aws/4.31.0/docs/resources/lambda_layer_version) | resource |
| [archive_file.default](https://registry.terraform.io/providers/hashicorp/archive/latest/docs/data-sources/file) | data source |

## Inputs

| Name | Description | Type | Default | Required |
|------|-------------|------|---------|:--------:|
| <a name="input_aws_region"></a> [aws\_region](#input\_aws\_region) | AWS region | `string` | n/a | yes |
| <a name="input_collector_configuration_yaml"></a> [collector\_configuration\_yaml](#input\_collector\_configuration\_yaml) | OTEL collector configuration file | `string` | `"collector.yaml"` | no |
| <a name="input_collector_configuration_zip"></a> [collector\_configuration\_zip](#input\_collector\_configuration\_zip) | ZIP-ed OTEL collector configuration file | `string` | `"collector.zip"` | no |
| <a name="input_environment"></a> [environment](#input\_environment) | n/a | `string` | n/a | yes |
| <a name="input_name"></a> [name](#input\_name) | n/a | `string` | `"otel_collector_config"` | no |
| <a name="input_organization"></a> [organization](#input\_organization) | n/a | `string` | n/a | yes |

## Outputs

| Name | Description |
|------|-------------|
| <a name="output_otel_collector_config_layer_arn"></a> [otel\_collector\_config\_layer\_arn](#output\_otel\_collector\_config\_layer\_arn) | The ARN of the Lambda Layer containing the OTEL Collector Configuration |
| <a name="output_otel_collector_configuration_yaml"></a> [otel\_collector\_configuration\_yaml](#output\_otel\_collector\_configuration\_yaml) | The OTEL Collector Configuration file name |
| <a name="output_otel_collector_layer_arn"></a> [otel\_collector\_layer\_arn](#output\_otel\_collector\_layer\_arn) | The ARN of the Lambda Layer running the OTEL Collector |
<!-- END_TF_DOCS -->