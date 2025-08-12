provider "aws" {
  region = var.aws_region
}

# VPC for the application
resource "aws_vpc" "email_sanitizer_vpc" {
  cidr_block           = "10.0.0.0/16"
  enable_dns_hostnames = true
  enable_dns_support   = true

  tags = {
    Name = "email-sanitizer-vpc"
  }
}

# Public subnet
resource "aws_subnet" "public_subnet" {
  vpc_id                  = aws_vpc.email_sanitizer_vpc.id
  cidr_block              = "10.0.1.0/24"
  map_public_ip_on_launch = true
  availability_zone       = "${var.aws_region}a"

  tags = {
    Name = "email-sanitizer-public-subnet"
  }
}

# Internet Gateway
resource "aws_internet_gateway" "igw" {
  vpc_id = aws_vpc.email_sanitizer_vpc.id

  tags = {
    Name = "email-sanitizer-igw"
  }
}

# Route table
resource "aws_route_table" "public_rt" {
  vpc_id = aws_vpc.email_sanitizer_vpc.id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.igw.id
  }

  tags = {
    Name = "email-sanitizer-public-rt"
  }
}

# Route table association
resource "aws_route_table_association" "public_rta" {
  subnet_id      = aws_subnet.public_subnet.id
  route_table_id = aws_route_table.public_rt.id
}

# Security group for EC2
resource "aws_security_group" "ec2_sg" {
  name        = "email-sanitizer-sg"
  description = "Security group for Email Sanitizer API"
  vpc_id      = aws_vpc.email_sanitizer_vpc.id

  # SSH access
  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # HTTP access
  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # HTTPS access
  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # Application port
  ingress {
    from_port   = 8080
    to_port     = 8080
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # Outbound traffic
  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "email-sanitizer-sg"
  }
}

# EC2 instance
resource "aws_instance" "email_sanitizer_instance" {
  ami                    = var.ami_id
  instance_type          = var.instance_type
  key_name               = var.key_name
  subnet_id              = aws_subnet.public_subnet.id
  vpc_security_group_ids = [aws_security_group.ec2_sg.id]

  root_block_device {
    volume_size = 20
    volume_type = "gp3"
  }

  user_data = <<-EOF
              #!/bin/bash
              apt-get update -y
              apt-get install -y docker.io docker-compose git
              systemctl start docker
              systemctl enable docker
              usermod -aG docker ubuntu
              
              # Clone the repository
              mkdir -p /app
              cd /app
              git clone https://github.com/SelfSend/email-sanitizer-api.git
              cd email-sanitizer-api
              
              # Run the setup script
              chmod +x deployment/scripts/setup.sh
              ./deployment/scripts/setup.sh
              EOF

  tags = {
    Name = "email-sanitizer-instance"
  }
}

# Elastic IP
resource "aws_eip" "email_sanitizer_eip" {
  instance = aws_instance.email_sanitizer_instance.id
  domain   = "vpc"

  tags = {
    Name = "email-sanitizer-eip"
  }
}

# Output the public IP
output "public_ip" {
  value = aws_eip.email_sanitizer_eip.public_ip
}

output "instance_id" {
  value = aws_instance.email_sanitizer_instance.id
}