output "public_ip" {
  description = "Public IP address of the EC2 instance"
  value       = aws_eip.email_sanitizer_eip.public_ip
}

output "instance_id" {
  description = "ID of the EC2 instance"
  value       = aws_instance.email_sanitizer_instance.id
}

output "api_url" {
  description = "URL to access the API"
  value       = "http://${aws_eip.email_sanitizer_eip.public_ip}:8080"
}

output "swagger_ui_url" {
  description = "URL to access the Swagger UI"
  value       = "http://${aws_eip.email_sanitizer_eip.public_ip}:8080/swagger-ui/"
}

output "graphql_playground_url" {
  description = "URL to access the GraphQL Playground"
  value       = "http://${aws_eip.email_sanitizer_eip.public_ip}:8080/api/v1/playground"
}