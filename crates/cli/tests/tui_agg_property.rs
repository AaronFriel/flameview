#![cfg(all(not(windows), not(miri)))]

use std::fs;
use std::path::PathBuf;

use flameview::loader::collapsed;
use flameview_cli::viewer::spark_tree_view::SparkTreeView;

#[test]
fn tui_agg_property() {
    let data_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "..",
        "..",
        "tests",
        "data",
        "small.txt",
    ]
    .iter()
    .collect();
    let data = fs::read(&data_path).unwrap();
    let tree = collapsed::load_slice(&data).unwrap();
    for cov in [0.5, 0.6, 0.7, 0.8, 0.9, 0.95, 0.99] {
        let view = SparkTreeView::new(&tree, 4, cov);
        let sum: f64 = view
            .rows()
            .iter()
            .filter(|r| r.depth == 1)
            .map(|r| r.cum_pct)
            .sum();
        assert!(
            (sum - 100.0).abs() <= 0.01,
            "coverage {cov} produced sum {sum}"
        );
    }
}
