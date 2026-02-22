#!/usr/bin/env bash
set -euo pipefail

repo_root="${1:-$(pwd)}"
input_path="${2:-tests/data/large.txt}"

if [[ ! -d "$repo_root" ]]; then
  echo "repo root not found: $repo_root" >&2
  exit 1
fi

cd "$repo_root"

if [[ ! -f Cargo.toml ]]; then
  echo "Cargo.toml not found in $repo_root" >&2
  exit 1
fi

if [[ ! -f "$input_path" ]]; then
  echo "folded input not found: $input_path" >&2
  exit 1
fi

cargo build --package flameview-cli --bin flameview >/dev/null

if [[ -t 0 && -t 1 ]]; then
  exec ./target/debug/flameview "$input_path"
else
  ./target/debug/flameview --summarize --max-lines 40 "$input_path"
fi
