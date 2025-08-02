#[test]
fn summarize_handles_large_counts() {
    let data = include_bytes!("../../../fuzz/artifacts/summarize/crash-365590c63669747b");
    // Ensure the fuzz-found input does not cause panics in parsing or summarizing.
    let tree = flameview::loader::collapsed::load_slice(data).expect("load fuzz artifact");
    let _ = tree.summarize(100, 1.0);
}
