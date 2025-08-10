#[test]
fn totals_basic() {
    let input = "a;b 3\na;c 2\n";
    let tree = flameview::load_stream(std::io::Cursor::new(input)).unwrap();
    assert_eq!(tree.total_samples(), 5);
}
