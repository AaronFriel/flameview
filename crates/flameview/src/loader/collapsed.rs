use std::io::Read;

use crate::arena::{FlameTree, NodeId};
use super::Error;

pub fn load<R: Read>(mut r: R) -> Result<FlameTree, Error> {
    let mut input = String::new();
    r.read_to_string(&mut input)?;
    let mut tree = FlameTree::new();

    for (line_no, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (stack, count_str) = match line.rsplit_once(' ') {
            Some(v) => v,
            None => return Err(Error::BadLine(line_no + 1)),
        };
        let count: u64 = match count_str.trim().parse() {
            Ok(v) => v,
            Err(_) => return Err(Error::BadLine(line_no + 1)),
        };
        let mut parent = tree.root();
        let mut path: Vec<NodeId> = Vec::with_capacity(stack.split(';').count() + 1);
        path.push(parent);
        for frame in stack.split(';') {
            let mut child = tree[parent].first_child;
            let mut found = None;
            while let Some(id) = child {
                if tree[id].name == frame {
                    found = Some(id);
                    break;
                }
                child = tree[id].next_sibling;
            }
            let id = match found {
                Some(id) => id,
                None => tree.insert_child(parent, frame, 0),
            };
            parent = id;
            path.push(id);
        }
        // increment counts
        tree.get_mut(parent).self_count = tree[parent].self_count.saturating_add(count);
        for id in path {
            tree.get_mut(id).total_count = tree[id].total_count.saturating_add(count);
        }
    }

    Ok(tree)
}
