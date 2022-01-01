terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 3.27"
    }
  }

  required_version = ">= 0.14.9"
}

provider "aws" {
  profile = var.profile
  region  = var.region
}

module "vpc" {
  source = "./vpc"
  name   = var.name
  region = var.region
}

module "sg" {
  source = "./sg"
  name   = var.name
  vpc_id = module.vpc.vpc_id
}

module "ps" {
  source       = "./ps"
  DATABASE_URL = var.DATABASE_URL
  AWS_ECR_URL  = var.AWS_ECR_URL
  AWS_REGION   = var.region
}

#module "s3" {
  #source = "./s3"
  #bucket = var.bucket
#}

module "ecr" {
  source = "./ecr"
  name   = var.name
}

module "iam" {
  source = "./iam"
}

module "ec2" {
  source                = "./ec2"
  name                  = var.name
  bucket                = var.bucket
  subnet_id             = element(module.vpc.public_subnets, 0)
  security_group_id     = module.sg.security_group_id
  key_path              = var.key_path
  instance_profile_name = module.iam.ecr_instance_profile.name
}

module "eip" {
  source      = "./eip"
  instance_id = module.ec2.instance_id
}

module "rds" {
  source            = "./rds"
  name              = var.name
  db_password       = var.db_password
  database_subnets  = module.vpc.database_subnets
  security_group_id = module.sg.security_group_id
}
