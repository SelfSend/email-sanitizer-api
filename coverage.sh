#!/bin/bash
# For grcov:
cargo clean
CARGO_INCREMENTAL=0 RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="cargo-test-%p-%m.profraw" cargo test
grcov . --binary-path ./target/debug/ -s . -t html -o ./coverage-report
open ./coverage-report/index.html