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

measure "setup" bash .agent/setup.sh
measure "stable-version" cargo +stable --version >/dev/null
measure "nightly-version" cargo +nightly --version >/dev/null

measure "build" cargo build --workspace --release --exclude flameview-fuzz
measure "test" cargo test --workspace --all-features --verbose
measure "clippy" cargo clippy --workspace --all-targets --all-features -- -D warnings
measure "fmt" cargo +nightly fmt --all -- --check
measure "actionlint" actionlint -color
