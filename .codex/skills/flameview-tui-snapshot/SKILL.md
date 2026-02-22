---
name: flameview-tui-snapshot
description: "Create and maintain snapshot-style regression tests for flameview interactive UI by replaying key sequences and asserting textual screen snapshots with Insta. Use when UI layout, key handling, filters, or heuristics change and you need deterministic regression coverage."
---

# Flameview TUI Snapshot

## Overview

Capture and maintain deterministic interactive UI snapshots so flameview TUI regressions are caught automatically when layout, filtering, or key handling changes.

## Snapshot Workflow

1. Add or update snapshot tests in `crates/cli/src/viewer/app.rs` by replaying key sequences through `App::on_key` and capturing `snapshot_text`.
2. Run snapshot checks to detect drift.
3. If changes are intentional, update snapshots and re-run tests.
4. Review snapshot diffs before finalizing.

Use the helper script:

    <repo-root>/.codex/skills/flameview-tui-snapshot/scripts/tui_snapshots.sh check <repo-root>

Update snapshots intentionally:

    <repo-root>/.codex/skills/flameview-tui-snapshot/scripts/tui_snapshots.sh update <repo-root>

List current snapshots:

    <repo-root>/.codex/skills/flameview-tui-snapshot/scripts/tui_snapshots.sh list <repo-root>

## Authoring Rules

- Keep datasets in tests small and deterministic.
- Drive interactions only through key events so snapshots model real user workflows.
- Prefer one snapshot per scenario (initial view, filter sequence, sort sequence).
- Include at least one assertion for auto-expand behavior to guard viewport-fill heuristics.

## Validation

After snapshot updates, run:

    cd <repo-root>
    cargo test --package flameview-cli

## References

- Read `references/snapshot-playbook.md` for example test patterns and review checklist.
