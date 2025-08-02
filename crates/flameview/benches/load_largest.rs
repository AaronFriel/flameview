use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flameview::loader::collapsed;

fn bench_load_largest(c: &mut Criterion) {
    let data = include_bytes!("../../../tests/data/perf-vertx-stacks-01-collapsed-all.txt");
    c.bench_function("load_largest", |b| {
        b.iter(|| {
            let tree = collapsed::load(black_box(&data[..])).unwrap();
            black_box(tree);
        });
    });
}

criterion_group!(benches, bench_load_largest);
criterion_main!(benches);
