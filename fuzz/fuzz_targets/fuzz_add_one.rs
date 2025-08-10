#![no_main]

use flameview::load_stream;
use libfuzzer_sys::fuzz_target;

// Feed arbitrary bytes to the collapsed loader and, when parsing
// succeeds, exercise the summarizer with parameters derived from
// the input. The fuzzer should never trigger a panic in either
// stage.
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    if let Ok(tree) = load_stream(Cursor::new(data)) {
        let max_lines = data.first().copied().unwrap_or(0) as usize;
        let coverage = data.get(1).map(|b| *b as f64 / 255.0).unwrap_or(0.5);
        let _ = tree.summarize(max_lines, coverage);
    }
});
