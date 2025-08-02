# Flameview

Flameview is a small Rust library and CLI for analyzing [folded stack](https://github.com/brendangregg/FlameGraph#2-folded-stacks) files produced by tools such as `perf`.

## Capturing stacks with `cargo flamegraph`

When profiling Rust applications you can leverage [`cargo flamegraph`](https://github.com/flamegraph-rs/flamegraph) to both generate an SVG flamegraph and extract the underlying folded stack data.

Use the `--post-process` option with a command like `tee` to save the folded stacks while still allowing `cargo flamegraph` to create the SVG:

```bash
cargo flamegraph --bin myapp --post-process 'tee stacks.folded'
```

This stores the stacks in `stacks.folded` which can be inspected with `flameview-cli`:

```bash
flameview-cli summarize stacks.folded
```

From Rust you can parse collapsed stacks already loaded in memory using
`flameview::loader::collapsed::load_slice`.

`cargo flamegraph` will still produce `flamegraph.svg` as usual.

## Benchmarking

To profile the loader on a substantial data set run the built-in
`load_largest` benchmark:

```bash
cargo flamegraph --package flameview --bench load_largest -- \
  --bench --post-process 'tee load_largest.folded'
```

Inspect the results with:

```bash
flameview-cli summarize load_largest.folded
```

The CLI also accepts `-` to read from standard input:

```bash
cat load_largest.folded | flameview-cli summarize -
```
