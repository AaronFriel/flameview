---
name: flameview-optimize-self
description: "Profile flameview itself under high sample rates and heavy folded inputs, identify parser/viewer hotspots, and validate improvements with benches/tests. Use when optimizing flameview performance or verifying regressions after profiler-related changes."
---

# Flameview Optimize Self

## Overview

Use this skill to run a repeatable self-optimization loop for flameview:

1. Generate a heavy folded input.
2. Profile flameview parsing at high sample rates.
3. Inspect hotspot summaries with flameview.
4. Re-run benches/tests to validate improvements.

## Heavy Input + High-Rate Profiling

Generate a large folded stack file:

    <repo-root>/.codex/skills/flameview-optimize-self/scripts/make_heavy_folded.sh \
      <repo-root> 160

Profile flameview on that file at a high sample rate:

    <repo-root>/.codex/skills/flameview-optimize-self/scripts/profile_self_high_rate.sh \
      <repo-root> \
      /tmp/flameview-self.folded \
      999

## Benchmark Validation

Run quick parser benchmarks for regression checks:

    <repo-root>/.codex/skills/flameview-optimize-self/scripts/bench_parser_fast.sh \
      <repo-root>

## Notes

- High sample rates can trigger perf dropped-sample warnings; prefer larger workloads to reduce noise.
- Use `flameview --summarize` to inspect generated folded outputs deterministically.
- For interactive UX inspection, pair this skill with `flameview-tui-driver`.

## References

- See `references/optimization-loop.md` for a concise optimization checklist.
