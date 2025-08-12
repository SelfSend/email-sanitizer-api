#!/bin/bash
set -e

# Navigate to the project directory
cd /app/email-sanitizer-api

# Pull latest changes
git pull

# Rebuild and restart containers
cd deployment/docker
docker-compose down
docker-compose build --no-cache
docker-compose up -d

echo "Email Sanitizer API has been updated successfully!"