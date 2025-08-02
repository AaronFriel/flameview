use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use cfg_if::cfg_if;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use flameview::loader::collapsed;

fn bench_load_collapsed(c: &mut Criterion) {
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/data");

    cfg_if! {
        if #[cfg(feature = "bench-fast")] {
            let names = vec![
                "perf-vertx-stacks-01-collapsed-all.txt".to_string(),
                "perf-java-stacks-01-collapsed-all.txt".to_string(),
            ];
            let warm_up = Duration::from_millis(10);
            let measure = Duration::from_millis(50);
            let sample_size = Some(10);
        } else {
            let names = fs::read_dir(&data_dir)
                .unwrap()
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                        Some(entry.file_name().into_string().unwrap())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            let warm_up = Duration::from_secs(1);
            let measure = Duration::from_secs(1);
            let sample_size = None::<usize>;
        }
    }

    let mut group = c.benchmark_group("load_collapsed");
    group.warm_up_time(warm_up);
    group.measurement_time(measure);
    if let Some(size) = sample_size {
        group.sample_size(size);
    }
    for name in names {
        let path = data_dir.join(&name);
        let bytes = fs::read(path).unwrap();
        group.bench_function(BenchmarkId::new("load", &name), move |b| {
            let data = bytes.clone();
            b.iter(|| {
                let tree = collapsed::load(black_box(data.as_slice())).unwrap();
                black_box(tree);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_load_collapsed);
criterion_main!(benches);
