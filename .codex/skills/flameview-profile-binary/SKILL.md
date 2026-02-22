---
name: flameview-profile-binary
description: "Profile Rust binaries on Linux using cargo flamegraph/perf custom events, keep folded stacks, and inspect them with flameview. Use when you need repeatable CPU/custom-event profiling with minimal dependencies, or when converting allocation profiles into flameview-ingestible folded stacks."
---

# Flameview Profile Binary

## Overview

Use this skill to produce folded stacks from Rust binaries and immediately inspect hotspots with flameview.

Assume `flameview` is installed in PATH. In this environment, installed location is:

    flameview

## Primary Workflow (recommended)

Use `cargo flamegraph` with `--post-process 'tee ...'` so one run produces both:

- `flamegraph.svg` for visual overview.
- `*.folded` for flameview interactive/summary analysis.

Run:

    <repo-root>/.codex/skills/flameview-profile-binary/scripts/profile_with_flamegraph.sh \
      <repo-root> \
      bin myapp cpu

For bench targets in non-default packages:

    <repo-root>/.codex/skills/flameview-profile-binary/scripts/profile_with_flamegraph.sh \
      <repo-root> \
      --package flameview \
      bench load_largest cpu /tmp/load_largest.folded -- --bench

For custom perf events:

    <repo-root>/.codex/skills/flameview-profile-binary/scripts/profile_with_flamegraph.sh \
      <repo-root> \
      bin myapp event branch-misses:u

## Allocation Conversion Workflow

`dhat-heap.json` is the output file produced by Rust's `dhat` heap profiler. It contains allocation stack traces and allocation metrics (bytes, blocks, or lifetimes), not CPU time.

If you already have `dhat-heap.json`, convert it to folded format and inspect with flameview:

    <repo-root>/.codex/skills/flameview-profile-binary/scripts/convert_dhat_to_folded.sh \
      <repo-root> \
      /path/to/dhat-heap.json

Direct command form:

    dhat-to-flamegraph -f folded -m total -u bytes -o /tmp/alloc.folded /path/to/dhat-heap.json
    flameview --summarize /tmp/alloc.folded

Useful variants:

- Leak-focused: `-m end`
- Peak heap spike: `-m heap-max`
- Allocation count instead of bytes: `-u blocks`

Minimal instrumentation pattern for Rust binaries:

    #[cfg(feature = "dhat-heap")]
    #[global_allocator]
    static ALLOC: dhat::Alloc = dhat::Alloc;

    fn main() {
        #[cfg(feature = "dhat-heap")]
        let _profiler = dhat::Profiler::new_heap();
        // app code
    }

## Perf and allocations

`perf` is excellent for CPU and custom hardware/software events, but allocation profiling via `perf` is not the easiest default.

- Practical path: place uprobes on allocator symbols (for example `malloc`, `free`, `realloc`) and aggregate stacks manually.
- Tradeoff: higher setup friction, symbol/ABI sensitivity, and weaker out-of-box "bytes retained by stack" reporting than `dhat`.
- Recommendation: use `dhat` -> folded conversion for allocation investigations, and use `perf` primarily for CPU and custom event flamegraphs.

Example allocator-call tracing on glibc systems:

    sudo perf probe -x /usr/lib/x86_64-linux-gnu/libc.so.6 'malloc size=%di'
    sudo perf record -e probe_libc:malloc -g -- ./target/release/myapp
    perf script | inferno-collapse-perf > /tmp/malloc.folded
    flameview --summarize /tmp/malloc.folded

Note: this shows sampled allocator call stacks/cost concentration; it is not equivalent to retained-bytes accounting.

## Notes for Coding Agents

- If TTY interaction is limited, use `flameview --summarize` first for deterministic output.
- For UI regression checks after interaction changes, use the `flameview-tui-snapshot` skill.
- Prefer command outputs written to `/tmp/*.folded` or repo-local ignored paths.

## References

- See `references/tool-matrix.md` for dependency/metric tradeoffs.
