---
name: flameview-tui-driver
description: "Drive interactive flameview terminal sessions and verify UX behavior (columns, filters, navigation, auto-expand). Use when a task requires launching flameview on folded stacks, exercising key bindings, reproducing TUI bugs, or capturing manual UX validation steps."
---

# Flameview TUI Driver

## Overview

Launch flameview quickly on representative folded-stack datasets and exercise repeatable key workflows to validate interactive UX behavior after code changes.

## Run Workflow

1. Run the helper script to build and launch flameview in a TTY.
2. Use the key workflow below to validate navigation, filtering, and auto-expansion.
3. Record observed regressions or unexpected output with the exact dataset and keys used.

Run from any location:

    <repo-root>/.codex/skills/flameview-tui-driver/scripts/run_flameview_tui.sh <repo-root> tests/data/large.txt

The script builds `flameview` if needed and opens the TUI. If no TTY is available it falls back to summarize mode so debugging can continue non-interactively.

## Validation Key Sequence

Use this sequence after launch to smoke-test the interactive UX:

1. `j` / `k` to move selection.
2. `l` or `Enter` to expand a branch.
3. `h` to collapse.
4. `]` then `[` to adjust minimum total-percent filter.
5. `H` to toggle hot-only filtering.
6. `f` to toggle runtime-frame hiding.
7. `/` then text then `Enter` to apply search.
8. `n` / `p` to jump matches.
9. `s` to cycle sort modes.
10. `z` to re-run heuristic auto-expand.
11. `q` to exit.

## Useful Commands

Run with a specific folded file:

    <repo-root>/.codex/skills/flameview-tui-driver/scripts/run_flameview_tui.sh <repo-root> /path/to/file.folded

Run summary mode directly (non-interactive sanity):

    cd <repo-root>
    ./target/debug/flameview --summarize tests/data/small.txt

## References

- Read `references/keymap.md` for the current key map and status semantics.
