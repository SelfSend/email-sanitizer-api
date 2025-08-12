# Email Sanitizer API Deployment Guide

This document provides step-by-step instructions for deploying the Email Sanitizer API to AWS EC2 using Terraform.

## Prerequisites

1. AWS CLI installed and configured with appropriate permissions
2. Terraform installed (v1.0+)
3. SSH key pair created in AWS
4. Git installed

## Deployment Steps

### 1. Clone the Repository

```bash
git clone https://github.com/SelfSend/email-sanitizer-api.git
cd email-sanitizer-api
```

### 2. Configure AWS Credentials

Ensure your AWS credentials are configured:

```bash
aws configure
```

### 3. Create SSH Key Pair in AWS

If you don't already have an SSH key pair:

```bash
aws ec2 create-key-pair --key-name email-sanitizer-key --query 'KeyMaterial' --output text > email-sanitizer-key.pem
chmod 400 email-sanitizer-key.pem
```

### 4. Configure Terraform Variables

Edit `deployment/terraform/variables.tf` to customize your deployment:

```bash
# Example: Change the region
sed -i 's/us-east-1/us-west-2/' deployment/terraform/variables.tf

# Example: Change the instance type
sed -i 's/t3.medium/t3.small/' deployment/terraform/variables.tf

# Example: Change the key name to match your SSH key
sed -i 's/email-sanitizer-key/your-key-name/' deployment/terraform/variables.tf
```

### 5. Deploy with Terraform

```bash
cd deployment/terraform
terraform init
terraform plan
terraform apply
```

When prompted, type `yes` to confirm the deployment.

### 6. Access the Application

After deployment completes, Terraform will output:
- `public_ip`: The public IP address of your EC2 instance
- `api_url`: URL to access the API
- `swagger_ui_url`: URL to access the Swagger UI
- `graphql_playground_url`: URL to access the GraphQL Playground

### 7. SSH into the Instance

```bash
ssh -i /path/to/your-key.pem ubuntu@<public_ip>
```

### 8. Verify Deployment

Once connected to the instance, check the deployment status:

```bash
cd /app/email-sanitizer-api
docker-compose -f deployment/docker/docker-compose.yml ps
```

### 9. View Logs

```bash
docker-compose -f deployment/docker/docker-compose.yml logs -f app
```

## Updating the Application

To update the application with the latest code:

```bash
ssh -i /path/to/your-key.pem ubuntu@<public_ip>
cd /app/email-sanitizer-api
./deployment/scripts/update.sh
```

## Troubleshooting

### MongoDB Connection Issues

If the application can't connect to MongoDB:

```bash
# Check if MongoDB container is running
docker ps | grep mongodb

# Check MongoDB logs
docker-compose -f deployment/docker/docker-compose.yml logs mongodb
```

### Redis Connection Issues

If the application can't connect to Redis:

```bash
# Check if Redis container is running
docker ps | grep redis

# Check Redis logs
docker-compose -f deployment/docker/docker-compose.yml logs redis
```

### Application Not Accessible

If you can't access the application:

1. Check if the application is running:
   ```bash
   docker-compose -f deployment/docker/docker-compose.yml ps
   ```

2. Check application logs:
   ```bash
   docker-compose -f deployment/docker/docker-compose.yml logs app
   ```

3. Verify security group settings in AWS console to ensure ports 8080, 80, and 443 are open.

## Cleaning Up

To destroy all resources created by Terraform:

```bash
cd deployment/terraform
terraform destroy
```

When prompted, type `yes` to confirm.