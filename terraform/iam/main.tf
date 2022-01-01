data "aws_iam_policy_document" "instance-assume-role-policy" {
  statement {
    actions = ["sts:AssumeRole"]

    principals {
      type        = "Service"
      identifiers = ["ec2.amazonaws.com"]
    }
  }
}

resource "aws_iam_policy" "ecr_policy" {
  name        = "ecr-policy"
  path        = "/"
  description = "ECR policy"
  policy      = jsonencode(local.ecr_policy)
}

resource "aws_iam_role" "role" {
  name = "ecr-role"

  assume_role_policy  = jsonencode(local.assume_role_policy)
  managed_policy_arns = [aws_iam_policy.ecr_policy.arn]
}

resource "aws_iam_instance_profile" "ecr_instance_profile" {
  name = "ecr-profile"
  role = aws_iam_role.role.name
}
