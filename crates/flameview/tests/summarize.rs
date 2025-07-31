#[test]
fn summary_limits_lines_and_coverage() {
    let data = include_str!("../../../tests/data/perl.txt");
    let tree = flameview::loader::collapsed::load(data.as_bytes()).unwrap();
    let out = tree.summarize(5, 0.90);
    let n = out.lines().count();
    assert!(n <= 6, "root + \u{2264}5 children");
    assert!(out.contains("(root)"));
    assert!(out.contains("  ")); // at least one indented line
    insta::assert_snapshot!(out, @r#"(root) (100.0%, 1000)
  foo (60.0%, 600)
    bar (40.0%, 400)
  baz (35.0%, 350)"#);
}
