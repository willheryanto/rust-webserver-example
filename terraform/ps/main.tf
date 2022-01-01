resource "aws_ssm_parameter" "database_url" {
  name        = "/netflux/production/DATABASE_URL"
  description = "The DATABASE_URL ENV"
  type        = "SecureString"
  value       = var.DATABASE_URL
}

resource "aws_ssm_parameter" "aws_region" {
  name        = "/netflux/production/AWS_REGION"
  description = "The REGION ENV"
  type        = "SecureString"
  value       = var.AWS_REGION
}

resource "aws_ssm_parameter" "aws_ecr_url" {
  name        = "/netflux/production/AWS_ECR_URL"
  description = "The AWS_ECR_URL ENV"
  type        = "SecureString"
  value       = var.AWS_ECR_URL
}
