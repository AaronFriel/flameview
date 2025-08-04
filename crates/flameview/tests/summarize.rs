#[test]
fn summary_limits_lines_and_coverage() {
    let data = include_str!("../../../tests/data/small.txt");
    let tree = flameview::loader::collapsed::load_slice(data.as_bytes()).unwrap();
    let out = tree.summarize(5, 0.90);
    let n = out.lines().count();
    assert!(n <= 6, "root + \u{2264}5 children");
    assert!(out.contains("(root)"));
    assert!(out.contains("  ")); // at least one indented line
    #[cfg(not(miri))]
    insta::assert_snapshot!(out);
}
