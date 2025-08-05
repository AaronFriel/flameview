use std::borrow::Cow;
use std::collections::HashSet;

use ratatui::prelude::*;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;

use flameview::arena::{FlameTree, NodeId};

pub const MAX_BAR: usize = 15;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RowState {
    Expanded,
    Collapsed,
    Other,
}

pub struct Row<'a> {
    pub depth: usize,
    pub name: Cow<'a, str>,
    pub cum_pct: f64,
    pub self_pct: f64,
    pub node_id: NodeId,
    state: RowState,
}

pub struct SparkTreeView<'a> {
    tree: &'a FlameTree,
    root: NodeId,
    rows: Vec<Row<'a>>,
    cursor: usize,
    show_spark: bool,
    open: HashSet<NodeId>,
    open_other: HashSet<NodeId>,
    max_lines: usize,
    coverage: f64,
}

impl<'a> SparkTreeView<'a> {
    pub fn new(tree: &'a FlameTree, max_lines: usize, coverage: f64) -> Self {
        let root = tree.root();
        let mut view = SparkTreeView {
            tree,
            root,
            rows: Vec::new(),
            cursor: 0,
            show_spark: true,
            open: HashSet::new(),
            open_other: HashSet::new(),
            max_lines,
            coverage,
        };
        view.open.insert(root);
        view.rebuild();
        view
    }

    fn rebuild(&mut self) {
        self.rows.clear();
        let total = self.tree[self.root].total_count as f64;
        self.build_rows(self.root, 0, total);
        if self.cursor >= self.rows.len() {
            self.cursor = self.rows.len().saturating_sub(1);
        }
    }

    fn build_rows(&mut self, id: NodeId, depth: usize, total: f64) {
        let node = &self.tree[id];
        let cum_pct = node.total_count as f64 / total * 100.0;
        let self_pct = node.self_count as f64 / total * 100.0;
        let has_children = node.first_child.is_some();
        let expanded = self.open.contains(&id);
        let state = if has_children {
            if expanded {
                RowState::Expanded
            } else {
                RowState::Collapsed
            }
        } else {
            RowState::Collapsed
        };
        let name = Cow::Borrowed(node.name.as_str());
        self.rows.push(Row {
            depth,
            name,
            cum_pct,
            self_pct,
            node_id: id,
            state,
        });
        if !has_children || !expanded {
            return;
        }

        // collect children
        let mut children = Vec::new();
        let mut child = node.first_child;
        while let Some(cid) = child {
            children.push(cid);
            child = self.tree[cid].next_sibling;
        }
        children.sort_by_key(|&cid| std::cmp::Reverse(self.tree[cid].total_count));

        let mut displayed = Vec::new();
        let mut shown = 0usize;
        let mut running = 0u64;
        if self.open_other.contains(&id) {
            displayed = children.clone();
        } else {
            for &cid in &children {
                if shown >= self.max_lines.saturating_sub(1) {
                    break;
                }
                if (running as f64) / (node.total_count as f64) >= self.coverage {
                    break;
                }
                running += self.tree[cid].total_count;
                displayed.push(cid);
                shown += 1;
            }
        }
        let mut undisplayed = Vec::new();
        if displayed.len() < children.len() {
            for cid in children {
                if !displayed.contains(&cid) {
                    undisplayed.push(cid);
                }
            }
        }

        for cid in displayed {
            self.build_rows(cid, depth + 1, total);
        }

        if !undisplayed.is_empty() && !self.open_other.contains(&id) {
            let mut tot: u64 = 0;
            let mut self_tot: u64 = 0;
            for cid in &undisplayed {
                tot += self.tree[*cid].total_count;
                self_tot += self.tree[*cid].self_count;
            }
            let cum_pct = tot as f64 / total * 100.0;
            let self_pct = self_tot as f64 / total * 100.0;
            self.rows.push(Row {
                depth: depth + 1,
                name: Cow::Borrowed("… Other"),
                cum_pct,
                self_pct,
                node_id: id,
                state: RowState::Other,
            });
        }
    }

    pub fn draw(&mut self, f: &mut Frame) {
        let mut lines: Vec<Line> = Vec::new();
        lines.push(Line::from(
            "cum% self%   function                     spark",
        ));
        for (i, row) in self.rows.iter().enumerate() {
            let indent = "  ".repeat(row.depth);
            let label = match row.state {
                RowState::Expanded => format!("{indent}▼ {}", row.name),
                RowState::Collapsed => {
                    if self.tree[row.node_id].first_child.is_some() {
                        format!("{indent}▶ {}", row.name)
                    } else {
                        format!("{indent}  {}", row.name)
                    }
                }
                RowState::Other => format!("{indent}{}", row.name),
            };
            let cum = format!("{:>4.0}", row.cum_pct.round());
            let selfp = format!("{:>4.0}", row.self_pct.round());
            let spark = if self.show_spark {
                make_spark(row.self_pct, row.cum_pct)
            } else {
                " ".repeat(MAX_BAR)
            };
            let text = format!("{cum} {selfp}   {label:<25}{spark:>MAX_BAR$}");
            let mut line = Line::from(text);
            if i == self.cursor + 1 {
                line.style = Style::default().reversed();
            }
            lines.push(line);
        }
        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, f.area());
    }

    pub fn up(&mut self) {
        if self.rows.is_empty() {
            return;
        }
        self.cursor = self.cursor.wrapping_add(self.rows.len() - 1) % self.rows.len();
    }

    pub fn down(&mut self) {
        if self.rows.is_empty() {
            return;
        }
        self.cursor = self.cursor.wrapping_add(1) % self.rows.len();
    }

    pub fn toggle_spark(&mut self) {
        self.show_spark = !self.show_spark;
    }

    pub fn expand(&mut self) {
        if self.rows.is_empty() {
            return;
        }
        let row = &self.rows[self.cursor];
        match row.state {
            RowState::Collapsed => {
                if self.tree[row.node_id].first_child.is_some() {
                    self.open.insert(row.node_id);
                    self.rebuild();
                }
            }
            RowState::Other => {
                self.open_other.insert(row.node_id);
                self.rebuild();
            }
            RowState::Expanded => {}
        }
    }

    pub fn collapse(&mut self) {
        if self.rows.is_empty() {
            return;
        }
        let row = &self.rows[self.cursor];
        match row.state {
            RowState::Expanded => {
                self.open.remove(&row.node_id);
                self.rebuild();
            }
            RowState::Collapsed | RowState::Other => {
                if row.depth > 0 {
                    let target_depth = row.depth - 1;
                    let mut idx = self.cursor;
                    while idx > 0 {
                        idx -= 1;
                        if self.rows[idx].depth == target_depth {
                            self.cursor = idx;
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn current_row(&self) -> Option<&Row<'a>> {
        self.rows.get(self.cursor)
    }

    pub fn rows(&self) -> &[Row<'a>] {
        &self.rows
    }
}

pub fn make_spark(self_pct: f64, cum_pct: f64) -> String {
    if cum_pct <= 0.0 {
        return " ".repeat(MAX_BAR);
    }
    let bar_chars = ((cum_pct / 100.0) * MAX_BAR as f64 + 1e-6).round() as usize;
    if bar_chars == 0 {
        return " ".repeat(MAX_BAR);
    }
    let self_chars = ((self_pct / cum_pct) * bar_chars as f64 + 1e-6).round() as usize;
    let self_chars = self_chars.min(bar_chars);
    let mut s = String::new();
    for i in 0..bar_chars {
        if i < self_chars {
            s.push('█');
        } else {
            s.push('░');
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_spark() {
        assert_eq!(make_spark(25.0, 50.0), "████░░░░");
    }
}
