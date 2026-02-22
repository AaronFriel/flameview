# Profiling Tool Matrix (Linux + Flameview)

## Best default

Use `cargo flamegraph` + `--post-process 'tee ...'`.

Why:

- Minimal additional dependencies when Rust + perf already exist.
- Produces folded stacks directly for flameview ingestion.
- Supports custom perf events via `-c "record -e ..."`.

## Practical alternatives

- `perf` + `inferno-collapse-perf`: good when you need direct control over perf script/collapse stages.
- `dhat` + `dhat-to-flamegraph`: workable for allocation-oriented profiles, but with extra conversion/setup.
- `samply`, `perf lock`, jemalloc-pprof stacks: potentially useful, but higher host/runtime setup friction in many environments.

## Allocation note (perf vs dhat)

- `perf` can trace allocator calls using uprobes (for example on `malloc`/`free`), but this is a manual workflow and usually not the easiest path to actionable allocation flamegraphs.
- `dhat` is generally easier for Rust allocation analysis because it directly records allocation stacks and can be converted to folded format with `dhat-to-flamegraph`.
- Use `perf` allocation tracing when you specifically need system-wide/event-level tracing and accept extra setup/overhead.

## Event examples

- CPU default: `cargo flamegraph --bin myapp --post-process 'tee cpu.folded'`
- Branch misses: `cargo flamegraph -c "record -e branch-misses:u -c 100 --call-graph dwarf,64000 -g" --bin myapp --post-process 'tee branch.folded'`
- Cycles: `cargo flamegraph -c "record -e cycles:u -c 100 --call-graph dwarf,64000 -g" --bin myapp --post-process 'tee cycles.folded'`
