variable "aws_region" {
  description = "AWS region to deploy resources"
  type        = string
  default     = "us-east-1"
}

variable "ami_id" {
  description = "AMI ID for the EC2 instance (Ubuntu 22.04 LTS)"
  type        = string
  default     = "ami-0c7217cdde317cfec" # Ubuntu 22.04 LTS in us-east-1
}

variable "instance_type" {
  description = "EC2 instance type"
  type        = string
  default     = "t3.medium" # 2 vCPU, 4 GiB memory
}

variable "key_name" {
  description = "SSH key pair name"
  type        = string
  default     = "email-sanitizer-key"
}