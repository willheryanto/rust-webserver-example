output "vpc_id" {
  value       = module.vpc.vpc_id
  description = "The VPC ID"
}

output "database_subnets" {
  value       = module.vpc.database_subnets
  description = "The database subnets"
}

output "private_subnets" {
  value       = module.vpc.private_subnets
  description = "The database subnets"
}

output "public_subnets" {
  value       = module.vpc.public_subnets
  description = "The database subnets"
}
