variable "name" {
  description = "The name of the stack"
  type        = string
}

variable "db_password" {
  description = "The password for the database"
  type        = string
}

variable "security_group_id" {
  description = "The security group ID"
  type        = string
}

variable "database_subnets" {
  description = "The database subnets"
  type        = list(string)
}
