## No default

variable "db_password" {
  description = "The password for the database"
  type        = string
}

variable "DATABASE_URL" {
  description = "The DATABASE_URL ENV"
  type        = string
}
variable "AWS_ECR_URL" {
  description = "The AWS_ECR_URL ENV"
  type        = string
}

## With default value

variable "name" {
  description = "The name of the stack"
  type        = string
  default     = "netflux-service"
}

variable "profile" {
  description = "The profile to use for the AWS CLI"
  type        = string
  default     = "default"
}

variable "bucket" {
  description = "The name of the main bucket"
  type        = string
  default     = "netflux-service-bucket"
}

variable "region" {
  description = "The region to use for the AWS CLI"
  type        = string
  default     = "ap-southeast-1"
}

variable "key_path" {
  description = "The path you store they key"
  type        = string
  default     = "../key.pem"
}

