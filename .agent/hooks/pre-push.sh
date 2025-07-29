#!/usr/bin/env bash
set -euxo pipefail

bash scripts/setup.sh
cargo +stable --version >/dev/null
cargo +nightly --version >/dev/null

cargo build --workspace --release --exclude flameview-fuzz
cargo test  --workspace --all-features --verbose
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo +nightly fmt --all -- --check
actionlint -color

chmod +x .agent/hooks/pre-push.sh
