resource "aws_eip_association" "eip_assoc" {
  instance_id   = var.instance_id
  allocation_id = aws_eip.eip.id
}

resource "aws_eip" "eip" {
  vpc = true
}
