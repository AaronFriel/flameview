use std::io::Read;

use crate::arena::FlameTree;
use crate::loader::Error;

pub fn load<R: Read>(mut r: R) -> Result<FlameTree, Error> {
    let mut s = String::new();
    r.read_to_string(&mut s)?;
    let mut tree = FlameTree::new();
    let mut scratch = Vec::new();
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
