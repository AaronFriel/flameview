use std::io::{BufRead, BufReader, Cursor, Read};

use crate::arena::FlameTree;
use crate::loader::Error;

#[deprecated(note = "use load_stream instead")]
pub fn load<R: Read>(r: R) -> Result<FlameTree, Error> {
    load_stream(BufReader::new(r))
}

/// Load a flamegraph from an in-memory slice of UTF-8 bytes.
#[deprecated(note = "use load_stream instead")]
pub fn load_slice(data: &[u8]) -> Result<FlameTree, Error> {
    load_stream(Cursor::new(data))
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn load_str(s: &str) -> Result<FlameTree, Error> {
    load_stream(Cursor::new(s.as_bytes()))
}

/// Load a flamegraph from a buffered reader.
pub fn load_stream<R: BufRead>(mut r: R) -> Result<FlameTree, Error> {
    let mut tree = FlameTree::new();
    let mut buf = String::new();
    let mut line_no: usize = 0;
    loop {
        buf.clear();
        let n = r.read_line(&mut buf)?;
        if n == 0 {
            break;
        }
        line_no += 1;
        let line = buf.trim().to_owned();
        if line.is_empty() {
            continue;
        }
        let Some(space) = line.rfind(' ') else {
            return Err(Error::BadLine(line_no));
        };
        let (stack_str, cnt_str) = line.split_at(space);
        let count: u64 = cnt_str
            .trim_start()
            .parse()
            .map_err(|_| Error::BadLine(line_no))?;
        let mut parent = tree.root();
        for frame in stack_str.split(';') {
            let id = tree.get_or_insert_child(parent, frame);
            parent = id;
        }
        tree.add_samples(parent, count);
    }
    Ok(tree)
}
