<!-- BEGIN_TF_DOCS -->
## Requirements

| Name | Version |
|------|---------|
| <a name="requirement_aws"></a> [aws](#requirement\_aws) | 4.35.0 |

## Providers

| Name | Version |
|------|---------|
| <a name="provider_aws"></a> [aws](#provider\_aws) | 4.35.0 |

## Modules

No modules.

## Resources

| Name | Type |
|------|------|
| [aws_cloudwatch_log_group.this](https://registry.terraform.io/providers/hashicorp/aws/4.35.0/docs/resources/cloudwatch_log_group) | resource |
| [aws_cloudwatch_log_stream.this](https://registry.terraform.io/providers/hashicorp/aws/4.35.0/docs/resources/cloudwatch_log_stream) | resource |
| [aws_cloudwatch_metric_alarm.this](https://registry.terraform.io/providers/hashicorp/aws/4.35.0/docs/resources/cloudwatch_metric_alarm) | resource |
| [aws_iam_role.this](https://registry.terraform.io/providers/hashicorp/aws/4.35.0/docs/resources/iam_role) | resource |
| [aws_iam_role_policy.this](https://registry.terraform.io/providers/hashicorp/aws/4.35.0/docs/resources/iam_role_policy) | resource |
| [aws_iam_role_policy_attachment.lambda_xray](https://registry.terraform.io/providers/hashicorp/aws/4.35.0/docs/resources/iam_role_policy_attachment) | resource |
| [aws_lambda_event_source_mapping.this](https://registry.terraform.io/providers/hashicorp/aws/4.35.0/docs/resources/lambda_event_source_mapping) | resource |
| [aws_lambda_function.lambda](https://registry.terraform.io/providers/hashicorp/aws/4.35.0/docs/resources/lambda_function) | resource |
| [aws_iam_policy.lambda_xray](https://registry.terraform.io/providers/hashicorp/aws/4.35.0/docs/data-sources/iam_policy) | data source |
| [aws_iam_policy_document.assume](https://registry.terraform.io/providers/hashicorp/aws/4.35.0/docs/data-sources/iam_policy_document) | data source |
| [aws_iam_policy_document.this](https://registry.terraform.io/providers/hashicorp/aws/4.35.0/docs/data-sources/iam_policy_document) | data source |

## Inputs

| Name | Description | Type | Default | Required |
|------|-------------|------|---------|:--------:|
| <a name="input_appsync_arn"></a> [appsync\_arn](#input\_appsync\_arn) | ARN of the AWS AppSync | `string` | n/a | yes |
| <a name="input_appsync_host"></a> [appsync\_host](#input\_appsync\_host) | The AppSync host. | `string` | n/a | yes |
| <a name="input_aws_region"></a> [aws\_region](#input\_aws\_region) | AWS region | `string` | n/a | yes |
| <a name="input_cloudwatch_alarms_sns_arn"></a> [cloudwatch\_alarms\_sns\_arn](#input\_cloudwatch\_alarms\_sns\_arn) | The SNS arn from which to send alarms to | `string` | `""` | no |
| <a name="input_environment"></a> [environment](#input\_environment) | n/a | `string` | n/a | yes |
| <a name="input_name"></a> [name](#input\_name) | n/a | `string` | `"lambda-user-notifications-count"` | no |
| <a name="input_notifications_dynamo_db_stream_arn"></a> [notifications\_dynamo\_db\_stream\_arn](#input\_notifications\_dynamo\_db\_stream\_arn) | The DynamoDB stream from the table which stores notifications. | `string` | `""` | no |
| <a name="input_organization"></a> [organization](#input\_organization) | n/a | `string` | n/a | yes |
| <a name="input_otel_collector_config_layer_arn"></a> [otel\_collector\_config\_layer\_arn](#input\_otel\_collector\_config\_layer\_arn) | The ARN of the Lambda Layer containing the OTEL Collector Configuration | `string` | n/a | yes |
| <a name="input_otel_collector_configuration_yaml"></a> [otel\_collector\_configuration\_yaml](#input\_otel\_collector\_configuration\_yaml) | OTEL collector configuration file | `string` | `"collector.yaml"` | no |
| <a name="input_otel_collector_layer_arn"></a> [otel\_collector\_layer\_arn](#input\_otel\_collector\_layer\_arn) | The ARN of the Lambda Layer running the OTEL Collector | `string` | n/a | yes |
| <a name="input_rust_log"></a> [rust\_log](#input\_rust\_log) | Log level, should be info or debug, defaults to info | `string` | `"info"` | no |

## Outputs

| Name | Description |
|------|-------------|
| <a name="output_lambda_function_arn"></a> [lambda\_function\_arn](#output\_lambda\_function\_arn) | The ARN of the Lambda Function |
| <a name="output_lambda_function_invoke_arn"></a> [lambda\_function\_invoke\_arn](#output\_lambda\_function\_invoke\_arn) | The Invoke ARN of the Lambda Function |
| <a name="output_lambda_function_name"></a> [lambda\_function\_name](#output\_lambda\_function\_name) | The name of the Lambda Function |
<!-- END_TF_DOCS -->