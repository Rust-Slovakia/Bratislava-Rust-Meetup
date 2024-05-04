resource "aws_iam_role" "this" {
  name = "${local.full_name}_lambda"

  assume_role_policy = data.aws_iam_policy_document.assume.json
}

data "aws_iam_policy_document" "assume" {
  statement {
    actions = ["sts:AssumeRole"]

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }

    effect = "Allow"
  }
}

resource "aws_iam_role_policy" "this" {
  name   = "${local.full_name}_lambda_default"
  role   = aws_iam_role.this.id
  policy = data.aws_iam_policy_document.this.json
}

data "aws_iam_policy_document" "this" {
  statement {
    effect = "Allow"

    actions = [
      "logs:PutLogEvents",
      "logs:CreateLogStream",
      "logs:DescribeLogStreams",
      "logs:DescribeLogGroups"
    ]

    resources = [
      "${aws_cloudwatch_log_group.this.arn}:*"
    ]
  }
}

data "aws_iam_policy" "lambda_xray" {
  name = "AWSXRayDaemonWriteAccess"
}

resource "aws_iam_role_policy_attachment" "lambda_xray" {
  role       = aws_iam_role.this.name
  policy_arn = data.aws_iam_policy.lambda_xray.arn
}
