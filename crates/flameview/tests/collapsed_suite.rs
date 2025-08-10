#![cfg(not(miri))]
use std::fs;

#[test]
fn totals_match_fixture_names() {
    let mut paths: Vec<_> = fs::read_dir("../../tests/data")
        .unwrap()
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    if cfg!(feature = "test-fast") {
        paths.truncate(2);
    }

    for path in paths {
        let data = fs::read(&path).unwrap();
        let tree = flameview::load_stream(std::io::Cursor::new(data)).unwrap();
        if let Some(num) = path
            .file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.rsplit('-').next())
            .and_then(|s| s.parse::<u64>().ok())
        {
            assert_eq!(tree.total_samples(), num, "file {path:?}");
        }
    }
}
