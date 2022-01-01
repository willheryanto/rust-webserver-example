# EC2
module "ec2_instance" {
  source  = "terraform-aws-modules/ec2-instance/aws"
  version = "~> 3.0"

  name = var.name

  ami                         = local.ami
  instance_type               = local.instance_type
  associate_public_ip_address = true

  subnet_id              = var.subnet_id
  vpc_security_group_ids = [var.security_group_id]
  key_name               = aws_key_pair.ssh-key.key_name

  iam_instance_profile = var.instance_profile_name
  user_data            = <<-EOF
    #!/bin/bash
    set -ex
    sudo yum update -y
    sudo amazon-linux-extras install docker -y
    sudo amazon-linux-extras install nginx1 -y
    sudo yum install jq -y
    sudo service docker start
    sudo usermod -a -G docker ec2-user
    sudo su root
    aws ssm get-parameters-by-path --region ap-southeast-1 --path '/netflux/production' --with-decryption | jq -r '.Parameters[] | "\(.Name)=\(.Value)"' | sed 's/.*production\///g' > .env
    export $(cat .env | xargs)
    aws ecr get-login-password --region "$AWS_REGION" | docker login --username AWS --password-stdin "$AWS_ECR_URL"
    docker pull "$AWS_ECR_URL/netflux-service:latest"
    docker run -p 80:8000 -d $AWS_ECR_URL/netflux-service
  EOF
}

resource "tls_private_key" "ssh" {
  algorithm = "RSA"
  rsa_bits  = 4096
}

resource "aws_key_pair" "ssh-key" {
  key_name   = "ssh-key"
  public_key = tls_private_key.ssh.public_key_openssh
}

resource "local_file" "private_key" {
  content         = tls_private_key.ssh.private_key_pem
  filename        = var.key_path
  file_permission = "0600"
}
# IAM profile for the bucket

#data "aws_iam_policy_document" "instance-assume-role-policy" {
#statement {
#actions = ["sts:AssumeRole"]

#principals {
#type        = "Service"
#identifiers = ["ec2.amazonaws.com"]
#}
#}
#}

#resource "aws_iam_policy" "netflux_bucket_policy" {
#name        = "netflux-bucket-policy"
#path        = "/"
#description = "Allow "
#policy      = jsonencode(local.netflux_bucket_policy)
#}

#resource "aws_iam_role" "bucket_acess" {
#name = "netflux-bucket-role"

#assume_role_policy  = jsonencode(local.assume_role_policy)
#managed_policy_arns = [aws_iam_policy.netflux_bucket_policy.arn]

#tags = {
#tag-key = "netflux-env"
#}
#}

#resource "aws_iam_instance_profile" "profile" {
#name = "netflux-profile"
#role = aws_iam_role.bucket_acess.name
#}
