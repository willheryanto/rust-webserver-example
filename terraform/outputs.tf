output "instance_public_ip" {
  description = "Public IP address of the EC2 instance"
  value       = module.ec2.instance_public_ip
}

output "instance_public_dns" {
  description = "Public DNS of the EC2 instance"
  value       = module.ec2.instance_public_dns
}

output "rds_address" {
  description = "Address of the RDS"
  value       = module.rds.address
}

output "elastic_public_ip" {
  description = "Elastic public IP"
  value       = module.eip.elastic_public_ip
}

output "elastic_private_ip" {
  description = "Elastic private IP"
  value       = module.eip.elastic_private_ip
}
