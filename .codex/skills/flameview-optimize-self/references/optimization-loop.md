# Flameview Optimization Loop

## Loop

1. Capture baseline benchmarks:

       cargo bench --package flameview --features bench-fast --bench load_largest -- --noplot
       cargo bench --package flameview --features bench-fast --bench load_collapsed -- --noplot

2. Build heavy folded input and run high-rate self-profile.
3. Inspect hotspot summary from generated folded profile.
4. Implement one targeted change.
5. Re-run steps 1-3 and compare times/hotspots.
6. Run:

       cargo test --workspace --exclude flameview-fuzz
       cargo clippy --workspace --exclude flameview-fuzz --all-targets -- -D warnings

## Typical parser hotspots

- `loader::collapsed::load_stream`
- `FlameTree::get_or_insert_child` and `find_child`
- `FlameTree::add_samples`

Prefer low-risk improvements first (allocation reduction, fewer passes, cheaper string handling).
