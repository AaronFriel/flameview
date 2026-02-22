#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 4 ]]; then
  cat >&2 <<USAGE
usage: $0 <repo-root> [--package <pkg>] <bin|example|bench|test> <target-name> <cpu|event> [event-name] [out-folded] [-- <target-args...>]
example: $0 /repo bin myapp cpu
example: $0 /repo --package flameview bench load_largest cpu /tmp/load_largest.folded -- --bench
example: $0 /repo bin myapp event branch-misses:u /tmp/branch.folded -- --input foo
USAGE
  exit 2
fi

repo_root="$1"
shift

package_args=()
if [[ "${1:-}" == "--package" ]]; then
  if [[ $# -lt 2 ]]; then
    echo "--package requires a value" >&2
    exit 2
  fi
  package_args=(--package "$2")
  shift 2
fi

if [[ $# -lt 3 ]]; then
  echo "missing required arguments" >&2
  exit 2
fi

kind="$1"
target="$2"
mode="$3"
shift 3

case "$kind" in
  bin) target_flag="--bin" ;;
  example) target_flag="--example" ;;
  bench) target_flag="--bench" ;;
  test) target_flag="--test" ;;
  *)
    echo "invalid target kind: $kind" >&2
    exit 2
    ;;
esac

case "$mode" in
  cpu)
    event_name="cpu"
    out_folded="${1:-/tmp/${target}.cpu.folded}"
    [[ $# -gt 0 ]] && shift || true
    ;;
  event)
    event_name="${1:-cycles:u}"
    [[ $# -gt 0 ]] && shift || true
    safe_event="$(echo "$event_name" | tr ':/' '__')"
    out_folded="${1:-/tmp/${target}.${safe_event}.folded}"
    [[ $# -gt 0 ]] && shift || true
    ;;
  *)
    echo "invalid mode: $mode (expected cpu|event)" >&2
    exit 2
    ;;
esac

args=()
if [[ "${1:-}" == "--" ]]; then
  shift
  args=("$@")
fi

cd "$repo_root"

if ! command -v cargo-flamegraph >/dev/null; then
  echo "cargo-flamegraph not found. install with: cargo install flamegraph --locked" >&2
  exit 1
fi

cmd=(cargo flamegraph "${package_args[@]}")
if [[ "$mode" == "event" ]]; then
  cmd+=( -c "record -e ${event_name} -c 100 --call-graph dwarf,64000 -g" )
fi
cmd+=( "$target_flag" "$target" --post-process "tee ${out_folded}" )
if [[ ${#args[@]} -gt 0 ]]; then
  cmd+=( -- "${args[@]}" )
fi

echo "running: ${cmd[*]}"
"${cmd[@]}"

echo "folded output: ${out_folded}"
if [[ -x ./target/debug/flameview ]]; then
  echo "summary preview:"
  ./target/debug/flameview --summarize --max-lines 20 "$out_folded" | sed -n '1,40p'
else
  echo "note: ./target/debug/flameview not found; build with cargo build --package flameview-cli --bin flameview"
fi
