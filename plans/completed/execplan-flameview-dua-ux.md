# Flameview DUA-like Interactive UX With Heuristics and Auto-Expand

This ExecPlan is a living document. The sections `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes & Retrospective` must be kept up to date as work proceeds.

The repository does not contain a local `PLANS.md`; this plan follows the fallback rules from `$CODEX_HOME/skills/execplan/references/PLANS.md`.

## Purpose / Big Picture

After this change, `flameview` will provide an interactive terminal UX that feels closer to `dua i`: useful fixed columns, visible ranking signals, practical filters, and visual cues for where time is spent. Users should be able to quickly answer: “which paths dominate total time?”, “which nodes are leaf-hot?”, and “where do I drill next?” without leaving the TUI. The TUI will also auto-expand high-value subtrees to fill available vertical space so the first screen is immediately informative rather than mostly collapsed.

The user-visible proof is: running `flameview <folded-file>` opens a multi-column explorer with bars/sparklines, filter controls, and smart initial expansion that populates the viewport.

## Progress

- [x] (2026-02-22 07:00Z) Read `execplan` and `skill-creator` skill instructions, confirmed no repo-local `PLANS.md`, and created this ExecPlan.
- [x] (2026-02-22 07:00Z) Spawned mandatory ExecPlan watchdog handle `019c8425-8d61-7311-8fc4-fabe18da1ecb`.
- [x] (2026-02-22 07:05Z) Replaced viewer internals with a flattened-row state model, custom rendering, and explicit expansion state (`crates/cli/src/viewer/app.rs`).
- [x] (2026-02-22 07:05Z) Implemented dua-like multi-column view (`total`, `tot%`, `self%`, `spark`, `flags`, `frame`) with status/header bars.
- [x] (2026-02-22 07:05Z) Implemented interactive filtering/search/sort controls and contextual help footer.
- [x] (2026-02-22 07:05Z) Implemented heuristic auto-expand to populate viewport and exposed a manual re-run key (`z`).
- [x] (2026-02-22 07:06Z) Added snapshot-style interaction tests in `crates/cli/src/viewer/app.rs` and generated baseline Insta snapshots.
- [x] (2026-02-22 07:08Z) Created and validated two project-local skills under `.codex/skills/` (`flameview-tui-driver`, `flameview-tui-snapshot`).
- [x] (2026-02-22 07:08Z) Ran validation (`cargo test --workspace --exclude flameview-fuzz`, `cargo clippy --package flameview-cli --all-targets -- -D warnings`, snapshot script checks).

## Surprises & Discoveries

- Observation: Existing TUI is a minimal wrapper over `tui-tree-widget`, with only navigation and quit controls, so nearly all requested behavior requires replacing the current viewer app model.
  Evidence: `crates/cli/src/viewer/app.rs` currently stores only widget items/state and handles keys `q/h/arrows`.
- Observation: `viewer::tui_from_reader` already falls back to text summary when stdout is not a TTY, which is useful for test stability and should be preserved.
  Evidence: `crates/cli/src/viewer/mod.rs` checks `!io::stdout().is_terminal()` and prints summary.
- Observation: Snapshot assertions require a first-time snapshot materialization step; running with `--nocapture` can pollute snapshot output.
  Evidence: Initial `insta` run produced `.snap.new` files and mixed output; rerun with `INSTA_UPDATE=always` created stable snapshots.
- Observation: User explicitly rejected global skill installation; skills had to be relocated to project-local `.codex/skills`.
  Evidence: User feedback plus successful relocation and validation with no remaining `$CODEX_HOME/skills/flameview-*` directories.

## Decision Log

- Decision: Replace the `tui-tree-widget`-centric renderer with an internal flattened-row view model while keeping the same CLI entrypoint and non-TTY fallback behavior.
  Rationale: Requested UX requires richer columns, filter overlays, and custom heuristics that are hard to express cleanly through the current tree widget abstraction.
  Date/Author: 2026-02-22 / Codex.
- Decision: Include a separate skill for “drive flameview TUI” in `$CODEX_HOME/skills` using `skill-creator`, instead of embedding all operational guidance in repo docs.
  Rationale: User explicitly requested a reusable skill and this keeps run/interact knowledge reusable across sessions.
  Date/Author: 2026-02-22 / Codex.
- Decision: Add a second dedicated skill for snapshot-style TUI regression capture rather than merging snapshot guidance into the generic interaction skill.
  Rationale: Snapshot maintenance has distinct workflows (scripted keys, deterministic capture, review/update process) and should trigger independently.
  Date/Author: 2026-02-22 / Codex.
- Decision: Store both new skills in repository-local `.codex/skills/` instead of `~/.codex/skills`.
  Rationale: User requested no global installation and local skills keep this workflow scoped to the project.
  Date/Author: 2026-02-22 / Codex.

## Outcomes & Retrospective

Implemented outcome: flameview now has a denser interactive UX with ranked columns, filter/search/sort controls, heuristic flags, and auto-expand behavior that fills viewport space with informative descendants. Snapshot-style regression coverage exists for key-sequence interactions in `crates/cli/src/viewer/app.rs` with stored `.snap` artifacts, and two reusable local skills now support both manual TUI driving and snapshot maintenance.

What remains: no additional code changes are required for the requested scope in this ExecPlan. Future iteration can refine rendering aesthetics and add richer mini-graphs, but the requested behavior baseline is present and tested.

Lesson learned: snapshot capture for terminal-oriented UI is most robust when test helpers produce deterministic textual renderings (`snapshot_text`) and key sequences are replayed against app state directly rather than relying on terminal emulator output.

## Context and Orientation

`flameview` is a Rust workspace with:

- Core parser/tree model in `crates/flameview/src/*`.
- CLI in `crates/cli/src/*`.
- Viewer runtime in `crates/cli/src/viewer/mod.rs` and `crates/cli/src/viewer/app.rs`.

The current viewer is a simple tree UI. It parses folded stacks into `FlameTree`, then renders a tree widget. “Folded stacks” means one line per stack path with a count at the end (`frameA;frameB;frameC 42`). `FlameTree` stores each frame node with `self_count` (samples ending at that frame) and `total_count` (inclusive samples for subtree).

The requested UX asks for:

- Dua-like information density (clear columns and ranking).
- Useful filters (search and relevance controls).
- Flamegraph-like visual understanding (fast view of where time goes).
- Intelligent expansion so the first viewport is informative.

## Plan of Work

Milestone 1 replaces viewer internals with a custom state model. In `crates/cli/src/viewer/app.rs`, define a row model for each visible node containing node id, depth, display label, total/self percentages, normalized spark widths, and heuristic flags (for example: “hot leaf”, “heavy fanout”, “linear chain”). Track expansion state by node id and selected row index. Build a flattening pass from root to visible rows and keep it idempotent so it can be recomputed after every key event.

Milestone 2 implements rendering layers in `crates/cli/src/viewer/app.rs`: header (dataset summary and filter state), table body (columns), and footer/help strip (key bindings). Columns will include at least: inclusive samples, inclusive percent, self percent, spark bar, and frame label with indentation. Spark bars use Unicode block glyphs to convey total intensity at a glance.

Milestone 3 adds interactions in `App::on_key` and small helper methods: navigation, expand/collapse, manual and automatic expansion commands, threshold/filter toggles, and search mode input. Auto-expand heuristic should greedily expand highest-total descendants while visible rows are fewer than viewport body rows, with safeguards against infinite expansion. This must be callable on first draw and by a key command to recompute after filter changes.

Milestone 4 adds/updates tests under `crates/cli/tests` (and unit tests inside viewer module where practical) for row flattening, auto-expand behavior, filter correctness, and non-TTY fallback regressions. Keep tests deterministic by operating on in-memory folded data.

Milestone 5 adds snapshot-style interactive regression tests so a key sequence can be replayed and compared against a stored textual “screen model” snapshot (using `insta`). This provides a React/Insta-like guard against accidental UX regressions.

Milestone 6 creates reusable Codex skills using `skill-creator`:

- Create skill directory (hyphen-case name) in `$CODEX_HOME/skills/`.
- Add `flameview-tui-driver` `SKILL.md` with trigger-rich description and exact workflow to run flameview in TUI mode, load representative datasets, and exercise interactive controls.
- Add `flameview-tui-snapshot` `SKILL.md` focused on generating and reviewing interactive UI snapshots from scripted key sequences.
- Add helper scripts/resources only when necessary (for example scripted keystroke driving or fixture selection).
- Validate with `quick_validate.py`.

## Concrete Steps

All commands run from repository root `<repo-root>` unless stated otherwise.

1. Implement viewer model and renderer:

    cargo fmt --all

2. Run focused tests for viewer and parser-facing integrations:

    cargo test --package flameview-cli view loader_stream -- --nocapture

3. Run full CLI crate tests:

    cargo test --package flameview-cli

4. Run workspace checks excluding fuzz (matches current project workflow):

    cargo test --workspace --exclude flameview-fuzz
    cargo clippy --workspace --exclude flameview-fuzz --all-targets -- -D warnings

5. Manual UX validation in TTY:

    ./target/debug/flameview tests/data/large.txt

Expected observations include:

- Header and footer bars are visible.
- Multiple useful columns are displayed.
- The initial viewport is auto-expanded and not mostly empty.
- Search/filter controls update visible rows and counts.

6. Create/validate skill (outside repo):

    # Working dir: $CODEX_HOME/skills/.system/skill-creator
    scripts/init_skill.py flameview-tui-driver --path $CODEX_HOME/skills --resources scripts,references
    scripts/init_skill.py flameview-tui-snapshot --path $CODEX_HOME/skills --resources scripts,references
    scripts/quick_validate.py $CODEX_HOME/skills/flameview-tui-driver
    scripts/quick_validate.py $CODEX_HOME/skills/flameview-tui-snapshot

## Validation and Acceptance

Acceptance criteria are behavior-based:

- Running `flameview <folded-file>` in a terminal shows a columnar table view with at least five meaningful columns, including a visual spark bar.
- The first screen auto-expands high-value subtrees so the visible table is populated with informative descendants rather than only a collapsed root.
- Users can apply at least three useful filters interactively (search text, threshold, and one heuristic/class filter) and see immediate row updates.
- Keyboard help/status line advertises controls and current filter/sort state.
- Non-TTY invocation still prints textual summary output exactly as before.
- Tests and lint pass for touched crates.
- A reusable skill exists at `$CODEX_HOME/skills/flameview-tui-driver` and validates with `quick_validate.py`.
- A reusable snapshot skill exists at `$CODEX_HOME/skills/flameview-tui-snapshot` and validates with `quick_validate.py`.

## Idempotence and Recovery

Edits are additive and can be safely rerun. If TUI behavior regresses, keep non-TTY fallback untouched and use `cargo test --package flameview-cli view` to isolate viewer issues quickly. If auto-expand produces overwhelming output, set a maximum expansion budget tied to viewport height and keep root collapse/expand commands available for manual recovery.

Skill creation is idempotent when rerun after deleting the generated folder; validation is read-only.

## Artifacts and Notes

Key references during planning:

- `crates/cli/src/viewer/app.rs` (current minimal tree-widget implementation).
- `crates/cli/src/viewer/mod.rs` (TTY setup loop and non-TTY fallback).
- `crates/cli/tests/view.rs` and `crates/cli/tests/loader_stream.rs` (current behavior contracts).

Watchdog handle:

- `019c8425-8d61-7311-8fc4-fabe18da1ecb`

Validation evidence:

- `cargo test --workspace --exclude flameview-fuzz` passed.
- `cargo clippy --package flameview-cli --all-targets -- -D warnings` passed.
- `<repo-root>/.codex/skills/flameview-tui-snapshot/scripts/tui_snapshots.sh check <repo-root>` passed.
- `quick_validate.py` reported `Skill is valid!` for both `.codex/skills/flameview-tui-driver` and `.codex/skills/flameview-tui-snapshot`.

## Interfaces and Dependencies

The implementation will keep using:

- `ratatui` + `crossterm` for terminal rendering/input.
- `flameview::arena::FlameTree` as source-of-truth tree model.

Planned internal interfaces in `crates/cli/src/viewer/app.rs`:

- `struct RowMetrics` for per-row computed stats.
- `struct Filters` for search/threshold/class toggles.
- `fn rebuild_rows(&mut self, viewport_rows: usize)` to recompute visible rows after state changes.
- `fn auto_expand_to_fill(&mut self, viewport_rows: usize)` to apply heuristic expansion.
- `fn spark_bar(percent: f64, width: usize) -> String` for compact visual signal.

Change note (2026-02-22 07:00Z): Added explicit snapshot-regression milestone and a second dedicated skill (`flameview-tui-snapshot`) per user request for key-sequence UI snapshots.
Change note (2026-02-22 07:08Z): Marked implementation complete, recorded validation evidence, and documented relocation of new skills from global scope to repository-local `.codex/skills` per user instruction.
