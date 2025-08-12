FROM rust:1.75 as builder

WORKDIR /usr/src/app
COPY . .

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/email-sanitizer /app/email-sanitizer

# Copy the .env file template
COPY .env.example /app/.env

# Expose the port the app runs on
EXPOSE 8080

# Command to run the application
CMD ["./email-sanitizer"]