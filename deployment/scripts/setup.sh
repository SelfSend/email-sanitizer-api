#!/bin/bash
set -e

# Navigate to the project directory
cd /app/email-sanitizer-api

# Create .env file from example
cp .env.example .env

# Create directories for nginx
mkdir -p deployment/docker/nginx/ssl

# Generate self-signed SSL certificate (for development)
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout deployment/docker/nginx/ssl/nginx.key \
  -out deployment/docker/nginx/ssl/nginx.crt \
  -subj "/C=US/ST=State/L=City/O=Organization/CN=localhost"

# Update the main.rs file to bind to 0.0.0.0 instead of 127.0.0.1
sed -i 's/"127.0.0.1"/"0.0.0.0"/g' src/main.rs

# Build and start the containers
cd deployment/docker
docker-compose up -d

echo "Email Sanitizer API has been deployed successfully!"
echo "Access the API at http://$(curl -s http://169.254.169.254/latest/meta-data/public-ipv4):8080"
echo "Access the Swagger UI at http://$(curl -s http://169.254.169.254/latest/meta-data/public-ipv4):8080/swagger-ui/"