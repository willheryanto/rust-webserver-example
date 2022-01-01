output "elastic_public_ip" {
  description = "Elastic public IP"
  value       = aws_eip_association.eip_assoc.public_ip
}

output "elastic_private_ip" {
  description = "Elastic private IP"
  value       = aws_eip_association.eip_assoc.private_ip_address
}
