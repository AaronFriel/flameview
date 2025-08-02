use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flameview::loader::collapsed;
use std::time::Duration;

fn bench_load_largest(c: &mut Criterion) {
    let data = include_bytes!("../../../tests/data/perf-vertx-stacks-01-collapsed-all.txt");
    c.bench_function("load_largest", |b| {
        b.iter(|| {
            let tree = collapsed::load(black_box(&data[..])).unwrap();
            black_box(tree);
        });
    });
}

fn criterion() -> Criterion {
    let mut c = Criterion::default();
    if cfg!(feature = "bench-fast") {
        c = c
            .warm_up_time(Duration::from_millis(10))
            .measurement_time(Duration::from_millis(100))
            .sample_size(10);
    }
    c
}

criterion_group! {
    name = benches;
    config = criterion();
    targets = bench_load_largest
}

criterion_main!(benches);
