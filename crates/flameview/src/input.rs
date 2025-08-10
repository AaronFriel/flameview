use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Open a flamegraph input source, handling "-" as stdin.
pub fn open_source(path: &Path) -> io::Result<Box<dyn BufRead>> {
    if path.as_os_str() == "-" {
        let stdin = io::stdin();
        Ok(Box::new(BufReader::new(stdin.lock())))
    } else {
        let file = File::open(path)?;
        Ok(Box::new(BufReader::new(file)))
    }
}
