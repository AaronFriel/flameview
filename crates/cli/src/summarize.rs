use std::io::BufRead;

use flameview::loader::{self, collapsed};

/// Load a flamegraph from the given reader and return a textual summary.
pub fn summarize<R: BufRead>(
    reader: R,
    max_lines: usize,
    coverage: f64,
) -> Result<String, loader::Error> {
    let tree = collapsed::load_stream(reader)?;
    Ok(tree.summarize(max_lines, coverage))
}
