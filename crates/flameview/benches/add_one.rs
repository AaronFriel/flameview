use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flameview::add_one;

fn bench_add_one(c: &mut Criterion) {
    c.bench_function("add_one", |b| b.iter(|| add_one(black_box(41))));
}

criterion_group!(benches, bench_add_one);
criterion_main!(benches);
