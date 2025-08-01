use std::io::{BufRead, BufReader, Read};

use crate::arena::FlameTree;
use crate::loader::Error;

pub fn load<R: Read>(r: R) -> Result<FlameTree, Error> {
    let mut reader = BufReader::new(r);
    let mut line = String::new();
    let mut tree = FlameTree::new();
    let mut scratch = Vec::new();
    let mut line_no = 0;
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        line_no += 1;
        let line = line.trim();
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
        scratch.clear();
        let mut parent = tree.root();
        for frame in stack_str.split(';') {
            let id = tree.get_or_insert_child(parent, frame);
            scratch.push(id);
            parent = id;
        }
        tree.add_samples(parent, count);
    }
    Ok(tree)
}
