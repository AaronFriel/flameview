use std::collections::VecDeque;

use crate::arena::{FlameTree, NodeId};

impl FlameTree {
    /// Pretty-prints an indented overview of the flame tree.
    ///
    /// * `max_lines` – maximum non-root rows to emit (root is always shown)
    /// * `coverage`  – stop early once cumulative **self** samples
    ///   reach this fraction of total (0.0‒1.0). This avoids
    ///   double-counting inclusive time from sibling branches.
    ///
    /// Returns a ready-to-display `String`.
    pub fn summarize(&self, max_lines: usize, coverage: f64) -> String {
        let total = self[self.root()].total_count as f64;
        let mut out = String::new();
        out.push_str(&format!(
            "(root) (100.0%, {})",
            self[self.root()].total_count
        ));

        if total == 0.0 || max_lines == 0 || coverage <= 0.0 {
            return out;
        }

        let mut queue: VecDeque<(NodeId, usize)> = VecDeque::new();
        // gather root children sorted and push onto queue
        let mut children = Vec::new();
        let mut child = self[self.root()].first_child;
        while let Some(id) = child {
            children.push(id);
            child = self[id].next_sibling;
        }
        children.sort_by_key(|&id| std::cmp::Reverse(self[id].total_count));
        for id in children.into_iter().rev() {
            queue.push_front((id, 1));
        }

        let mut printed = 0usize;
        let mut cum_total = 0u64;

        while let Some((id, depth)) = queue.pop_front() {
            let node = &self[id];
            out.push('\n');
            out.push_str(&"  ".repeat(depth));
            out.push_str(&format!(
                "{} ({:.1}%, {})",
                node.name,
                node.total_count as f64 / total * 100.0,
                node.total_count
            ));
            printed += 1;
            cum_total += node.self_count;

            // push children of this node
            let mut ch = node.first_child;
            if ch.is_some() {
                let mut chvec = Vec::new();
                while let Some(cid) = ch {
                    chvec.push(cid);
                    ch = self[cid].next_sibling;
                }
                chvec.sort_by_key(|&cid| std::cmp::Reverse(self[cid].total_count));
                for cid in chvec.into_iter().rev() {
                    queue.push_front((cid, depth + 1));
                }
            }

            if printed >= max_lines || (cum_total as f64 / total) >= coverage {
                break;
            }
        }

        out
    }
}
