output "ecr_instance_profile" {
  value       = aws_iam_instance_profile.ecr_instance_profile
  description = "The ECR instance profile"
}
