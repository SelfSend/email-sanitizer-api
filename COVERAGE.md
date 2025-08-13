# Code Coverage Setup

## Local Coverage Testing

Run coverage locally:
```bash
./coverage.sh
```

This will:
- Clean previous coverage data
- Run tests with coverage instrumentation
- Generate HTML report at `coverage/index.html`
- Generate LCOV report at `coverage.lcov`
- Open the HTML report in your browser

## CI/CD Integration

Coverage is automatically generated and uploaded to Codecov on every push/PR via GitHub Actions.

## Configuration Files

- `.codecov.yml` - Codecov configuration with 80% target coverage
- `coverage.sh` - Local coverage generation script
- `.github/workflows/ci.yml` - CI pipeline with coverage upload

## Ignored Files

The following files are excluded from coverage:
- `src/main.rs` - Application entry point
- `src/lib.rs` - Library root
- `src/openapi.rs` - OpenAPI documentation
- `target/` - Build artifacts
- `tests/` - Test files

## Requirements

- `grcov` - Install with `cargo install grcov`
- Rust nightly toolchain (for coverage instrumentation)