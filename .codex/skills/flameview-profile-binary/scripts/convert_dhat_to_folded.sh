#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 ]]; then
  echo "usage: $0 <repo-root> <dhat-heap.json> [out-folded]" >&2
  exit 2
fi

repo_root="$1"
dhat_json="$2"
out_folded="${3:-/tmp/dhat.alloc.folded}"

if [[ ! -f "$dhat_json" ]]; then
  echo "dhat json not found: $dhat_json" >&2
  exit 1
fi

if ! command -v dhat-to-flamegraph >/dev/null; then
  echo "dhat-to-flamegraph not found; installing..."
  cargo install dhat-to-flamegraph --locked
fi

cd "$repo_root"
dhat-to-flamegraph -f folded -o "$out_folded" "$dhat_json"

echo "folded output: $out_folded"
if [[ -x ./target/debug/flameview ]]; then
  ./target/debug/flameview --summarize --max-lines 20 "$out_folded" | sed -n '1,40p'
fi
