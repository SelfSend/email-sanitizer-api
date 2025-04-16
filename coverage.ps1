# This script is used to generate code coverage reports for Rust projects using grcov on Windows.
# It sets up the environment variables, runs the tests, and generates an HTML report.
# Make sure to have grcov installed and available in your PATH.
# Usage: Run this script in PowerShell after navigating to your Rust project directory:
# `Set-ExecutionPolicy RemoteSigned -Scope Process  # Allow script execution`
# `.\coverage.ps1`
# Clean previous artifacts
# Clean environment
cargo clean
Remove-Item -Recurse -Force target\profraw -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force coverage-report -ErrorAction SilentlyContinue

# Create directories
New-Item -ItemType Directory -Path target\profraw -Force | Out-Null
New-Item -ItemType Directory -Path coverage-report -Force | Out-Null

# Set environment variables
$env:CARGO_INCREMENTAL = "0"
$env:RUSTFLAGS = "-C instrument-coverage -C opt-level=0"
$env:LLVM_PROFILE_FILE = "target/profraw/cargo-test-%p-%m.profraw"

# Build first (catch compilation errors early)
cargo build --tests

# Run tests
cargo test --tests --no-run

# Generate coverage data
cargo test --tests --no-fail-fast -- --nocapture

# Process coverage (Windows-specific paths)
grcov `
  target\profraw `
  --binary-path target\debug\deps `
  --source-dir . `
  --llvm-path "$(rustc --print sysroot)/lib/rustlib/x86_64-pc-windows-msvc/bin" `
  --output-type html `
  --branch `
  --ignore "/*" `
  --ignore "*/tests/*" `
  --ignore-not-existing `
  -o coverage-report

# Open report if successful
if (Test-Path coverage-report/index.html) {
  start coverage-report/index.html
}
else {
  Write-Host "Checking for profraw files..."
  Get-ChildItem target\profraw | Format-Table Name
  Write-Error "Failed to generate coverage report"
}