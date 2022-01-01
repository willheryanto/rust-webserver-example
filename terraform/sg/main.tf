module "security_group" {
  source  = "terraform-aws-modules/security-group/aws"
  version = "~> 4"

  name        = var.name
  description = "Security group"
  vpc_id      = var.vpc_id

  ingress_cidr_blocks      = ["0.0.0.0/0", "10.0.0.0/16"]
  ingress_ipv6_cidr_blocks = ["::/0"]
  ingress_rules            = ["all-all"]
  egress_rules             = ["all-all"]
}
