#!/usr/bin/env bash
set -euo pipefail

action="${1:-check}"
repo_root="${2:-$(pwd)}"

if [[ ! -d "$repo_root" ]]; then
  echo "repo root not found: $repo_root" >&2
  exit 1
fi

cd "$repo_root"

if [[ ! -f Cargo.toml ]]; then
  echo "Cargo.toml not found in $repo_root" >&2
  exit 1
fi

case "$action" in
  check)
    cargo test --package flameview-cli viewer::app::tests
    ;;
  update)
    INSTA_UPDATE=always cargo test --package flameview-cli viewer::app::tests
    ;;
  list)
    find crates/cli/src/viewer/snapshots -maxdepth 1 -name '*.snap' -type f | sort
    ;;
  *)
    echo "usage: $0 {check|update|list} [repo-root]" >&2
    exit 2
    ;;
esac
