use std::fs;
use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use flameview::loader::collapsed;

fn bench_load_collapsed(c: &mut Criterion) {
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/data");
    let mut group = c.benchmark_group("load_collapsed");
    for entry in fs::read_dir(&data_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            let name = entry.file_name().into_string().unwrap();
            let bytes = fs::read(path).unwrap();
            group.bench_function(BenchmarkId::new("load", &name), move |b| {
                let data = bytes.clone();
                b.iter(|| {
                    let tree = collapsed::load(black_box(data.as_slice())).unwrap();
                    black_box(tree);
                });
            });
        }
    }
    group.finish();
}

criterion_group!(benches, bench_load_collapsed);
criterion_main!(benches);
