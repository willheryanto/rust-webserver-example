module "db" {
  source  = "terraform-aws-modules/rds/aws"
  version = "~> 2.0"

  identifier = var.name

  publicly_accessible = true

  engine               = "postgres"
  engine_version       = "12.8"
  family               = "postgres12"
  major_engine_version = "12"
  instance_class       = "db.t2.micro"

  allocated_storage = 20
  storage_encrypted = false

  name     = "netflux"
  username = "netflux"
  password = var.db_password
  port     = "5432"


  backup_retention_period = 0
  skip_final_snapshot     = true
  deletion_protection     = false

  performance_insights_enabled = true

  multi_az               = false
  subnet_ids             = var.database_subnets
  vpc_security_group_ids = [var.security_group_id]

  maintenance_window = "Mon:00:00-Mon:03:00"
  backup_window      = "03:00-06:00"
}
