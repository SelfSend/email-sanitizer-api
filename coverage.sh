#!/bin/bash

# Clean previous coverage data
cargo clean
rm -rf coverage/
rm -f *.profraw
mkdir -p coverage/profraw

# Set environment variables for coverage
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-C instrument-coverage"
export LLVM_PROFILE_FILE="coverage/profraw/cargo-test-%p-%m.profraw"

# Run tests with coverage
echo "Running tests with coverage..."
cargo test

# Generate coverage report
echo "Generating coverage report..."
grcov coverage/profraw/ \
    --binary-path ./target/debug/deps/ \
    -s . \
    -t html \
    --branch \
    --ignore-not-existing \
    --ignore '../*' \
    --ignore "/*" \
    --ignore 'target/*' \
    --ignore 'src/main.rs' \
    --ignore 'src/lib.rs' \
    --ignore 'src/openapi.rs' \
    -o coverage/

# Generate lcov for Codecov
grcov coverage/profraw/ \
    --binary-path ./target/debug/deps/ \
    -s . \
    -t lcov \
    --branch \
    --ignore-not-existing \
    --ignore '../*' \
    --ignore "/*" \
    --ignore 'target/*' \
    --ignore 'src/main.rs' \
    --ignore 'src/lib.rs' \
    --ignore 'src/openapi.rs' \
    -o coverage.lcov

# Clean up profraw files
rm -rf coverage/profraw/

echo "Coverage report generated at coverage/index.html"
echo "LCOV report generated at coverage.lcov"

# Open coverage report if on macOS/Linux with GUI
if command -v xdg-open > /dev/null; then
    xdg-open coverage/index.html
elif command -v open > /dev/null; then
    open coverage/index.html
fi