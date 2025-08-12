#!/bin/bash
set -e

echo "🔍 Running CI checks locally..."

echo "📝 Checking formatting..."
cargo fmt --all -- --check

echo "🔧 Running Clippy..."
cargo clippy -- -D warnings

echo "🧪 Running tests..."
cargo test

echo "🏗️  Building project..."
cargo build

echo "✅ All CI checks passed!"