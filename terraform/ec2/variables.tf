variable "name" {
  description = "The name of the stack"
  type        = string
}

variable "bucket" {
  description = "The name of the main bucket"
  type        = string
}

variable "subnet_id" {
  description = "The subnet ID"
  type        = string
}

variable "security_group_id" {
  description = "The security group ID"
  type        = string
}

variable "key_path" {
  description = "The path you store they key"
  type        = string
}

variable "instance_profile_name" {
  description = "The name of the instance profile"
  type        = string
}

locals {
  ami           = "ami-0dc5785603ad4ff54"
  instance_type = "t2.micro"
}
