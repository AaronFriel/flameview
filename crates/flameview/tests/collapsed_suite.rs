use std::fs;
#[cfg(not(miri))]
#[test]
fn totals_match_fixture_names() {
    for entry in fs::read_dir("../../tests/data").unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            let data = fs::read(&path).unwrap();
            let tree = flameview::loader::collapsed::load(data.as_slice()).unwrap();
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
}
