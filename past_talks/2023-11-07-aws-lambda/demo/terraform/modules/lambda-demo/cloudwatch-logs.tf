
resource "aws_cloudwatch_log_group" "this" {
  name = "/aws/lambda/${local.full_name}"
}

resource "aws_cloudwatch_log_stream" "this" {
  name           = local.full_name
  log_group_name = aws_cloudwatch_log_group.this.name
}