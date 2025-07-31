#[test]
fn totals_basic() {
    let input = "a;b 3\na;c 2\n";
    let tree = flameview::loader::collapsed::load(input.as_bytes()).unwrap();
    assert_eq!(tree.total_samples(), 5);
}
