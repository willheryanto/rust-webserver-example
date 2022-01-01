resource "aws_s3_bucket" "bucket" {
  bucket        = var.bucket
  force_destroy = true
  acl           = "private"

  tags = {
    Name = "Env bucket"
  }

  server_side_encryption_configuration {
    rule {
      apply_server_side_encryption_by_default {
        sse_algorithm = "AES256"
      }
    }
  }
}

resource "aws_s3_bucket_public_access_block" "bucket_access" {
  bucket = aws_s3_bucket.bucket.id

  block_public_acls   = true
  block_public_policy = true
  ignore_public_acls  = true
}
