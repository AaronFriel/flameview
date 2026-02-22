# Flameview

Flameview is a small Rust library and CLI for analyzing [folded stack](https://github.com/brendangregg/FlameGraph#2-folded-stacks) files produced by tools such as `perf`.

## Recommended Linux profiling workflow

For Linux, the easiest low-dependency path we recommend is:

- `cargo flamegraph` for capture.
- `--post-process 'tee ...'` to keep folded stacks.
- `flameview` to inspect folded stacks interactively or as summary text.

This works for default CPU sampling and for custom `perf` events.

### 1) CPU time (default)

When profiling Rust applications you can leverage [`cargo flamegraph`](https://github.com/flamegraph-rs/flamegraph) to both generate an SVG flamegraph and extract the underlying folded stack data.

`cargo flamegraph` requires the Linux `perf` tool to be installed and accessible
in your `PATH`. Use the `--post-process` option with a command like `tee` to save the folded stacks while still allowing `cargo flamegraph` to create the SVG:

```bash
cargo flamegraph --bin myapp --post-process 'tee stacks.folded'
```

This stores the stacks in `stacks.folded` which can be inspected with `flameview`:

```bash
flameview --summarize stacks.folded
```

From Rust you can parse collapsed stacks via the streaming loader
`flameview::load_stream`.

`cargo flamegraph` will still produce `flamegraph.svg` as usual.

### 2) Custom perf events (for example, branch misses)

You can use `cargo flamegraph -c` to pass a custom `perf record` command. This
lets you profile non-default metrics while still generating folded stacks for
flameview.

```bash
cargo flamegraph \
  -c "record -e branch-misses:u -c 100 --call-graph dwarf,64000 -g" \
  --bin myapp \
  --post-process 'tee branch-misses.folded'
```

Inspect with flameview:

```bash
flameview --summarize branch-misses.folded
```

### 3) Manual perf + inferno collapse pipeline

If you already have `perf.data` or want explicit control over `perf script`,
collapse with `inferno-collapse-perf` and inspect in flameview:

```bash
perf record -F 99 -g -- ./target/release/myapp
perf script | inferno-collapse-perf > myapp.folded
flameview --summarize myapp.folded
```

## Allocation-oriented profiling (moderate setup)

For allocation-heavy analysis, `dhat-rs` plus `dhat-to-flamegraph` is a
practical path to folded stacks:

```bash
# inside your instrumented app build/run flow, produce dhat-heap.json
# then convert:
dhat-to-flamegraph -f folded -o alloc.folded dhat-heap.json
flameview --summarize alloc.folded
```

This is more setup than CPU/custom-event profiling, which is why the default
recommendation remains `cargo flamegraph` + optional custom perf events.

## Benchmarking

To profile the loader on a substantial data set run the built-in
`load_largest` benchmark:

```bash
cargo flamegraph --package flameview --bench load_largest -- \
  --bench --post-process 'tee load_largest.folded'
```

Inspect the results with:

```bash
flameview --summarize load_largest.folded
```

The CLI also accepts `-` to read from standard input:

```bash
cat load_largest.folded | flameview --summarize -
```

## Profiling flameview itself

To stress-test flameview parsing and profile with a high sample rate:

```bash
# Create a large folded input from the fixture corpus.
tmp_folded="$(mktemp /tmp/flameview-self-XXXXXX.folded)"
for i in $(seq 1 160); do
  cat tests/data/large.txt >> "$tmp_folded"
done

# Profile flameview parsing that file at high frequency.
./target/debug/cargo-flameview \
  --package flameview-cli \
  --bin flameview \
  --frequency 999 \
  -- \
  --summarize "$tmp_folded"
```

The command prints the generated folded profile path (`folded: ...`), which can
be re-opened with `flameview --summarize` for hotspot inspection.

For a quicker smoke test that completes in under half a minute, enable the
`bench-fast` feature. This runs a small, representative subset of benches with
reduced warmup and measurement times:

```bash
cargo bench --package flameview --features bench-fast
```

## Large tests

Some integration tests exercise the loader on a 200&nbsp;MiB sample to guard
against excessive memory use. These are disabled by default; run
`cargo test --all-features` to include them.
