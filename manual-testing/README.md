# Manual Testing Guide

## Authentication Flow

1. **Register a user**: Run `POST-register.http` to create a user and get an API key
2. **Copy the API key**: From the response, copy the `api_key` value
3. **Use in authenticated requests**: Replace `YOUR_API_KEY_HERE` in auth test files

## Test Files

### Authentication Tests
- `POST-register.http` - Register new user and generate API key
- `POST-register_duplicate.http` - Test duplicate registration (should fail)
- `POST-register_invalid.http` - Test invalid registration data
- `POST-auth_invalid_key.http` - Test invalid API key formats

### Email Validation (Authenticated)
- `POST-email_valid_with_auth.http` - Validate email with API key
- `POST-emails_bulk_with_auth.http` - Bulk validation with API key
- `GET-job_status_with_auth.http` - Job status with API key

### Email Validation (Public)
- `POST-email_valid.http` - Public email validation
- `POST-email_invalid_*.http` - Various invalid email tests

## API Key Format

Generated API keys have the format: `{hash_prefix}.{jwt_token}`
- Hash prefix: First 16 chars of SHA-256(email + password_hash)
- JWT token: Contains email and expiration (30 days)