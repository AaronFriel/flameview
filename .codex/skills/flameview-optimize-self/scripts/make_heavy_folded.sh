#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "usage: $0 <repo-root> [repeat-count] [out-path]" >&2
  exit 2
fi

repo_root="$1"
repeat_count="${2:-160}"
out_path="${3:-/tmp/flameview-self.folded}"

cd "$repo_root"
base_file="tests/data/large.txt"
if [[ ! -f "$base_file" ]]; then
  echo "missing fixture: $base_file" >&2
  exit 1
fi

: > "$out_path"
for _ in $(seq 1 "$repeat_count"); do
  cat "$base_file" >> "$out_path"
done

ls -lh "$out_path"
