#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "usage: $0 <repo-root>" >&2
  exit 2
fi

repo_root="$1"
cd "$repo_root"

cargo bench --package flameview --features bench-fast --bench load_largest -- --noplot
cargo bench --package flameview --features bench-fast --bench load_collapsed -- --noplot
