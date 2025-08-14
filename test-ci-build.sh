#!/bin/bash

# Test CI Build Sequence Locally
# This script simulates the exact build sequence that will run in CI

set -e  # Exit on any error

echo "🧹 Cleaning build artifacts..."
cargo clean

echo "🔨 Building project (Debug)..."
cargo build --verbose

echo "🔨 Building project (Release)..."
cargo build --release --verbose

echo "🧪 Running tests..."
cargo test --verbose

echo "✅ CI build sequence completed successfully!"
echo "This indicates the CI pipeline should work correctly."