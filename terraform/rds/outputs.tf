output "address" {
  value       = module.db.this_db_instance_address
  description = "Connect to the database at this endpoint"
}

output "port" {
  value       = module.db.this_db_instance_port
  description = "The port the database is listening on"
}
