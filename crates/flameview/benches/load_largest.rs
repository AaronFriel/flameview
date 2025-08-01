use std::fs;
use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flameview::loader::collapsed;

fn bench_load_largest(c: &mut Criterion) {
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/data");
    let path = data_dir.join("perf-vertx-stacks-01-collapsed-all.txt");
    let bytes = fs::read(path).unwrap();
    c.bench_function("load_vertx_all", |b| {
        let data = bytes.clone();
        b.iter(|| {
            let tree = collapsed::load(black_box(data.as_slice())).unwrap();
            black_box(tree);
        });
    });
}

criterion_group!(benches, bench_load_largest);
criterion_main!(benches);
