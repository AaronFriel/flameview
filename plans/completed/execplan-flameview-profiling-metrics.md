# Linux Profiling Toolchain Evaluation and Flameview Self-Optimization

This ExecPlan is a living document. The sections `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes & Retrospective` must be kept up to date as work proceeds.

The repository does not include a local `PLANS.md`; this plan follows `$CODEX_HOME/skills/execplan/references/PLANS.md`. This plan is complementary to `execplan-flameview-dua-ux.md` and focuses on profiling workflows, documentation, skills, and optimization.

## Purpose / Big Picture

After this change, a contributor should be able to profile Rust binaries on Linux for CPU and non-CPU metrics (including custom perf events), route results into flameview, and use one clear recommended workflow with low setup friction. They should also be able to run project-local skills that automate profiling and snapshot-driven interaction workflows.

This plan also applies the chosen workflow to optimize flameview itself by profiling flameview running against heavy inputs and high sampling settings, then implementing measurable improvements.

## Progress

- [x] (2026-02-22 07:27Z) Started new ExecPlan for profiling-toolchain evaluation and optimization scope.
- [x] (2026-02-22 07:27Z) Spawned mandatory watchdog handle `019c843f-1b7e-7b90-99d8-7c449f94b448`.
- [x] (2026-02-22 07:33Z) Completed hands-on feasibility checks for CPU/custom-event and allocation/non-time candidates, including success/failure notes and dependency friction.
- [x] (2026-02-22 07:34Z) Selected default recommendation: `cargo flamegraph` + `--post-process 'tee ...'` with custom `-c "record -e ..."` when needed.
- [x] (2026-02-22 07:35Z) Updated `README.md` with recommended Linux workflow, custom perf-event examples, manual inferno pipeline, allocation conversion notes, and self-profiling example.
- [x] (2026-02-22 07:38Z) Added repository-local skills: `.codex/skills/flameview-profile-binary` and `.codex/skills/flameview-optimize-self` with executable scripts and references.
- [x] (2026-02-22 07:39Z) Captured coding-agent ergonomics in skills (`--summarize` deterministic fallback, script wrappers, snapshot-skill cross-reference).
- [x] (2026-02-22 07:41Z) Applied workflow to flameview self-profile at high sample rates and implemented parser optimization in `crates/flameview/src/loader/collapsed.rs` plus `insert_child` zero-delta fast path.
- [x] (2026-02-22 07:42Z) Ran full validation (`fmt`, workspace tests, workspace clippy) and recorded benchmark/profile evidence.

## Surprises & Discoveries

- Observation: There is no public repository at `flamegraph-rs/cargo-flameview`; only `flamegraph-rs/flamegraph` exists publicly.
  Evidence: `git ls-remote https://github.com/flamegraph-rs/cargo-flameview.git HEAD` returned “Repository not found”.
- Observation: Repository-local research clone strategy is already in place and gitignored.
  Evidence: `.gitignore` contains `.research/` and `.research/flamegraph` currently resolves to commit `0672d88`.
- Observation: `inferno` CLI utilities were not initially present on PATH even though the crate is used as a dependency.
  Evidence: `inferno-collapse-perf`/`inferno-flamegraph` were missing before running `cargo install inferno --locked --version 0.11.21`.
- Observation: a per-parent hash-map child index in `FlameTree` regressed benchmark performance despite reducing theoretical lookup complexity.
  Evidence: `load_largest` jumped from ~223 µs baseline to ~354 µs; rollback restored/improved performance.
- Observation: reducing per-line allocation/string trimming in loader parsing gave measurable wins.
  Evidence: post-change benches improved against initial baseline (`load_largest` ~223 µs -> ~175 µs; collapsed fixture benches improved similarly).

## Decision Log

- Decision: Use a separate ExecPlan file for profiling/tooling work rather than extending the completed TUI ExecPlan.
  Rationale: This scope includes external-tool evaluation, documentation policy, skill authoring, and performance tuning beyond the TUI UX implementation.
  Date/Author: 2026-02-22 / Codex.
- Decision: Keep new skills repository-local in `.codex/skills/` only.
  Rationale: User explicitly requested not to install these skills globally.
  Date/Author: 2026-02-22 / Codex.
- Decision: Recommend `cargo flamegraph` with custom `-c` perf command as the default Linux workflow.
  Rationale: It consistently produced folded stacks consumable by flameview, required fewer moving pieces than alternatives, and supports both default CPU and custom perf events.
  Date/Author: 2026-02-22 / Codex.
- Decision: Prefer low-risk parser optimizations (allocation reduction + zero-delta fast path) over structural tree-index refactors.
  Rationale: The structural refactor regressed measured performance; low-risk string-path changes produced measurable improvements with no test regressions.
  Date/Author: 2026-02-22 / Codex.

## Outcomes & Retrospective

Completed outcomes:

- Added end-to-end Linux profiling guidance to `README.md` with copy-paste commands for CPU, custom events, inferno pipeline, allocation conversion, and flameview self-profiling.
- Added two new local skills with scripts:
  - `.codex/skills/flameview-profile-binary/`
  - `.codex/skills/flameview-optimize-self/`
- Applied and validated flameview self-optimization changes:
  - `crates/flameview/src/loader/collapsed.rs`: removed per-line owned string allocation; reduced trimming overhead.
  - `crates/flameview/src/arena.rs`: skipped ancestry propagation when `self_cnt == 0` in `insert_child`.
- Verified with full test/lint pass and benchmark evidence.

Remaining work:

- None for this requested scope. Future work can explore deeper data-structure changes for `find_child` only if accompanied by robust benchmark guardrails to avoid regressions.

## Context and Orientation

Relevant repository areas:

- `README.md`: user-facing profiling guidance.
- `crates/cli/src/profile.rs`: current `cargo-flameview` flow (`perf record` + `perf script` + inferno collapse).
- `crates/flameview/src/loader/collapsed.rs`: folded-stack parser hot path.
- `crates/flameview/benches/*.rs`: benchmark surfaces.
- `.codex/skills/`: repository-local skill directory for automation workflows.
- `.research/`: gitignored research sandbox and external checkouts.

Terms used in this plan:

- “Custom perf event”: Linux `perf record -e <event>` selection such as branch misses instead of default CPU-clock sampling.
- “Folded stack”: one-line stack representation (`a;b;c N`) used by flamegraph and flameview tooling.
- “Allocation flamegraph”: profile where stack sample value reflects allocation bytes/count rather than CPU time.

## Plan of Work

First, run practical feasibility checks for each candidate toolchain on this Linux environment and score each by install complexity, external dependencies, profile precision/control, and conversion friction into flameview folded stacks. Favor workflows that require only `perf` + Rust toolchain + existing project dependencies.

Second, document the best workflow in `README.md` with copy-paste commands, including a custom-event example and explicit ingestion through `flameview`.

Third, create project-local skills to execute this workflow repeatably. One skill will focus on profiling binaries and producing folded stacks/summary output; another will capture “agent usage” guidance for interacting with flameview (TTY + snapshot-driven fallback).

Fourth, apply the workflow to flameview itself using high sample rates and heavy inputs. Identify hotspots, implement at least one optimization in flameview code, and show evidence from tests/bench/profile output.

## Concrete Steps

All commands run from `<repo-root>` unless otherwise stated.

1. Evaluate toolchain feasibility:

    - Verify/install needed tools (`cargo flamegraph`, `inferno`, selected allocation tool candidates).
    - Run smoke profiles on `flameview` binary and on benchmark target.
    - Capture whether output is directly folded, easily convertible, or SVG-only.

2. Choose default recommendation and document rationale in this plan.

3. Update docs:

    - Edit `README.md` to add a “Recommended Linux profiling workflow” section.
    - Include CPU and custom-event examples and flameview ingestion commands.

4. Create local skills:

    - Add `.codex/skills/flameview-profile-binary/`.
    - Add `.codex/skills/flameview-optimize-self/` (specialized self-profiling/optimization loop skill).
    - Validate with `quick_validate.py`.

5. Optimize flameview using chosen profiling loop:

    - Gather hotspot evidence from heavy runs.
    - Implement targeted optimization(s).
    - Re-run validation and compare evidence.

6. Final validation:

    - `cargo fmt --all`
    - `cargo test --workspace --exclude flameview-fuzz`
    - `cargo clippy --workspace --exclude flameview-fuzz --all-targets -- -D warnings`

## Validation and Acceptance

Acceptance requires all of the following:

- A clear recommendation exists in `README.md` for Linux profiling with at least one custom perf-event workflow and flameview ingestion.
- New repository-local skill(s) exist and validate with `quick_validate.py`.
- At least one flameview optimization is implemented and justified with profile/benchmark evidence.
- Core test/lint commands pass after changes.
- The workflow is demonstrated on flameview itself at high sample rates or a heavy repeated workload.

## Idempotence and Recovery

Research commands are repeatable and mostly read-only; profiling outputs should be written into ignored or temporary paths (`.research/` or `/tmp`). If a profiling run fails due permissions/tooling, keep commands and errors documented and continue with best-supported fallback.

Skill creation is idempotent when editing existing directories in place and re-validating.

## Artifacts and Notes

Watchdog handle:

- `019c843f-1b7e-7b90-99d8-7c449f94b448`

Initial state evidence:

- `.research/` is gitignored.
- `.research/flamegraph` exists at commit `0672d88`.

Execution evidence:

- CPU/custom-event ingestion succeeded:
  - `cargo flamegraph --package flameview --bench load_largest --post-process 'tee /tmp/flameview-load-larges.folded'`
  - `perf record ... | inferno-collapse-perf > ...folded`
  - `flameview --summarize ...folded`
- Non-CPU feasibility highlights:
  - `dhat` + `dhat-to-flamegraph` produced folded output.
  - `samply` install succeeded but runtime capture blocked by `perf_event_paranoid=2`.
  - `perf lock` contention tracepoint unavailable on this host.
- Benchmarks (initial baseline vs final optimized):
  - `load_largest`: ~223 µs baseline -> ~175 µs final.
  - `load_collapsed/perf-vertx...`: ~225 µs baseline -> ~160 µs final.
  - `load_collapsed/perf-java...`: ~51 µs baseline -> ~31 µs final.

## Interfaces and Dependencies

Primary expected dependencies:

- Linux `perf`
- `cargo flamegraph` (from `flamegraph-rs/flamegraph`)
- Existing `flameview` CLI binaries

Possible optional dependencies under evaluation:

- `inferno` CLI utilities
- Allocation profilers (`jemalloc_pprof`, `jeprofl`, or alternatives)

Planned new interfaces:

- Skill command wrappers in `.codex/skills/*/scripts/` to run profiling commands consistently.
- README command examples producing folded stacks ingestible by `flameview`.

Change note (2026-02-22 07:27Z): Created new profiling-focused ExecPlan with required living sections and concrete validation criteria.
Change note (2026-02-22 07:42Z): Marked all milestones complete, recorded recommendation and optimization decisions, and added benchmark/validation evidence.
