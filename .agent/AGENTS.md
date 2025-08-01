# 🤖 AGENTS — Local CI Mirror

## Mandatory checks (block merge if failing)
    bash .agent/check.sh

## Performance analysis (optional)
    cargo bench
    cargo flamegraph --bin flameview-cli

## One-time setup
    bash .agent/setup.sh

Keep this file in sync with `.github/workflows/` to avoid CI drift.

This AGENTS.md is a living document. Record open tasks, repository structure,
and tips that help future contributors. Keep entries brief yet informative.

## Repository overview
- `crates/cli/src/main.rs` – command-line interface
- `crates/flameview/src/lib.rs` – core library (`add_one` example)
- `fuzz/` – fuzzing harness (`fuzz_targets/fuzz_add_one.rs`)
- `.agent/` – local CI helpers
- `.agent/setup.sh` – install Rust toolchains and tools

## Coding guidelines
- Start with tests before implementing features
- Place tests in separate files (e.g. `mod tests;` or `tests/` directory)
- Aim for correctness before optimizations
- Document style feedback here so it's not forgotten
- Update this file whenever assumptions change or new tasks arise

This document is short-term memory. Run `bash .agent/setup.sh` once to install
tools, then `bash .agent/check.sh` before pushing.

### Benchmarks
- All benches live in `crates/flameview/benches/` and are executed in CI via
  `cargo bench --package flameview`. Adding a new benchmark does not require
  modifying workflow files.

### Notes
- Miri runs tests in an isolated environment without access to OS operations like opening directories. Any test that reads from the filesystem should either be skipped with `#[cfg(not(miri))]` or rewritten to avoid directory reads when running under Miri.
- Run `cargo clippy -- -D warnings` with `RUSTFLAGS="--cfg miri"` to compile tests with the same cfg flags used by Miri. This catches unused imports or other compile errors before running Miri.

### Snapshot testing
- Use [`insta`](https://insta.rs/) for inline snapshots.
- Prefer inline snapshots when verifying CLI output or other textual representations.
- After modifying snapshots, run `cargo insta review` to accept new outputs.
- Snapshot assertions do not work under Miri; guard them with `#[cfg(not(miri))]`.

### Maintaining this file
- When updating AGENTS.md, provide context and reasoning that future contributors can apply. Avoid notes that only explain a workaround without describing the underlying issue.
