#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 ]]; then
  echo "usage: $0 <repo-root> <folded-input> [frequency]" >&2
  exit 2
fi

repo_root="$1"
folded_input="$2"
frequency="${3:-999}"

cd "$repo_root"
if [[ ! -f "$folded_input" ]]; then
  echo "input folded file not found: $folded_input" >&2
  exit 1
fi

cargo build --package flameview-cli --bin flameview --bin cargo-flameview >/dev/null

./target/debug/cargo-flameview \
  --package flameview-cli \
  --bin flameview \
  --frequency "$frequency" \
  -- \
  --summarize "$folded_input"

echo "Tip: inspect generated profile with"
echo "  ./target/debug/flameview --summarize <printed-folded-path>"
