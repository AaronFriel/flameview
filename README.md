# Flameview

Flameview is a small Rust library and CLI for analyzing [folded stack](https://github.com/brendangregg/FlameGraph#2-folded-stacks) files produced by tools such as `perf`.

## Capturing stacks with `cargo flamegraph`

When profiling Rust applications you can leverage [`cargo flamegraph`](https://github.com/flamegraph-rs/flamegraph) to both generate an SVG flamegraph and extract the underlying folded stack data.

`cargo flamegraph` requires the Linux `perf` tool to be installed and accessible
in your `PATH`. Use the `--post-process` option with a command like `tee` to save the folded stacks while still allowing `cargo flamegraph` to create the SVG:

```bash
cargo flamegraph --bin myapp --post-process 'tee stacks.folded'
```

This stores the stacks in `stacks.folded` which can be inspected with `flameview-cli`:

```bash
flameview-cli summarize stacks.folded
```

`cargo flamegraph` will still produce `flamegraph.svg` as usual.

## Benchmarks

Run Criterion benches to evaluate `flameview` performance. The `load_largest`
benchmark focuses on a single large stack file which makes it a good target for
profiling:

```bash
cargo bench --bench load_largest
```
