use std::io::{Error as IoError, ErrorKind, Read};

use crate::arena::FlameTree;
use crate::loader::Error;

pub fn load<R: Read>(mut r: R) -> Result<FlameTree, Error> {
    let mut buf = Vec::new();
    r.read_to_end(&mut buf)?;
    load_slice(&buf)
}

/// Load a flamegraph from an in-memory slice of UTF-8 bytes.
pub fn load_slice(data: &[u8]) -> Result<FlameTree, Error> {
    let s = std::str::from_utf8(data)
        .map_err(|e| Error::Io(IoError::new(ErrorKind::InvalidData, e)))?;
    load_str(s)
}

fn load_str(s: &str) -> Result<FlameTree, Error> {
    let mut tree = FlameTree::new();
    for (line_no, raw) in s.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let Some(space) = line.rfind(' ') else {
            return Err(Error::BadLine(line_no + 1));
        };
        let (stack_str, cnt_str) = line.split_at(space);
        let count: u64 = cnt_str
            .trim_start()
            .parse()
            .map_err(|_| Error::BadLine(line_no + 1))?;
        let mut parent = tree.root();
        for frame in stack_str.split(';') {
            let id = tree.get_or_insert_child(parent, frame);
            parent = id;
        }
        tree.add_samples(parent, count);
    }
    Ok(tree)
}
