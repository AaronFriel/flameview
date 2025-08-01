#!/usr/bin/env bash
set -euxo pipefail

measure() {
    local label="$1"; shift
    echo "--- $label ---"
    local start=$(date +%s)
    "$@"
    local end=$(date +%s)
    echo "$label took $((end - start))s"
}

measure "build" cargo build --workspace --release --exclude flameview-fuzz
measure "test" cargo test --workspace --all-features --verbose
measure "clippy" cargo clippy --workspace --all-targets --all-features -- -D warnings
measure "fmt" cargo +nightly fmt --all
# Compile tests under the same cfg flags Miri uses
RUSTFLAGS="--cfg miri" measure cargo clippy --workspace --all-targets --all-features -- -D warnings
measure "actionlint" actionlint -color