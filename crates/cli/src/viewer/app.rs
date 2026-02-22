use std::collections::{HashMap, HashSet};

use ratatui::crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::*;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Wrap};

use flameview::arena::{FlameTree, NodeId};

const DEFAULT_VIEWPORT_ROWS: usize = 20;
const HOT_SELF_PCT: f64 = 1.0;

pub enum Action {
    None,
    Exit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InputMode {
    Normal,
    Search,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SortMode {
    TotalDesc,
    SelfDesc,
    NameAsc,
}

impl SortMode {
    fn label(self) -> &'static str {
        match self {
            SortMode::TotalDesc => "total",
            SortMode::SelfDesc => "self",
            SortMode::NameAsc => "name",
        }
    }

    fn next(self) -> Self {
        match self {
            SortMode::TotalDesc => SortMode::SelfDesc,
            SortMode::SelfDesc => SortMode::NameAsc,
            SortMode::NameAsc => SortMode::TotalDesc,
        }
    }
}

#[derive(Clone, Debug)]
struct Filters {
    query: String,
    min_total_pct: f64,
    hot_only: bool,
    hide_runtime: bool,
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            query: String::new(),
            min_total_pct: 0.0,
            hot_only: false,
            hide_runtime: true,
        }
    }
}

#[derive(Clone, Debug)]
struct RowMetrics {
    id: NodeId,
    depth: usize,
    label: String,
    total_count: u64,
    self_count: u64,
    child_count: usize,
    total_pct: f64,
    self_pct: f64,
    has_children: bool,
    expanded: bool,
    flags: String,
}

pub struct App<'a> {
    tree: &'a FlameTree,
    rows: Vec<RowMetrics>,
    expanded: HashSet<NodeId>,
    parent_by_id: HashMap<NodeId, NodeId>,
    selected: usize,
    scroll: usize,
    input_mode: InputMode,
    show_help: bool,
    filters: Filters,
    sort_mode: SortMode,
    viewport_rows: usize,
    total_samples: u64,
    status_msg: String,
    match_indices: Vec<usize>,
    match_cursor: usize,
}

impl<'a> App<'a> {
    pub fn new(tree: &'a FlameTree) -> Self {
        let mut expanded = HashSet::new();
        expanded.insert(tree.root());

        let mut app = Self {
            tree,
            rows: Vec::new(),
            expanded,
            parent_by_id: build_parent_map(tree),
            selected: 0,
            scroll: 0,
            input_mode: InputMode::Normal,
            show_help: false,
            filters: Filters::default(),
            sort_mode: SortMode::TotalDesc,
            viewport_rows: DEFAULT_VIEWPORT_ROWS,
            total_samples: tree.total_samples(),
            status_msg: String::from("ready (runtime hidden; press f to toggle)"),
            match_indices: Vec::new(),
            match_cursor: 0,
        };
        app.auto_expand_to_fill(DEFAULT_VIEWPORT_ROWS);
        app.rebuild_rows(DEFAULT_VIEWPORT_ROWS);
        app
    }

    pub fn draw(&mut self, f: &mut Frame) {
        let footer_height = if self.show_help { 2 } else { 1 };
        let areas = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(3),
            Constraint::Length(footer_height),
        ])
        .split(f.area());

        let table_view_rows = areas[1].height.saturating_sub(3) as usize;
        self.viewport_rows = table_view_rows.max(1);
        self.rebuild_rows(self.viewport_rows);

        let header = Paragraph::new(self.header_line()).style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
        f.render_widget(header, areas[0]);

        let (start, end) = self.table_window_bounds();
        let visible_rows = &self.rows[start..end];

        let table_rows = visible_rows.iter().map(|row| self.render_table_row(row));
        let header_row = Row::new([
            Cell::from("total"),
            Cell::from("tot%"),
            Cell::from("self%"),
            Cell::from("spark"),
            Cell::from("flags"),
            Cell::from("frame"),
        ])
        .style(Style::default().add_modifier(Modifier::BOLD));

        let table = Table::new(
            table_rows,
            [
                Constraint::Length(11),
                Constraint::Length(6),
                Constraint::Length(6),
                Constraint::Length(10),
                Constraint::Length(5),
                Constraint::Min(20),
            ],
        )
        .header(header_row)
        .column_spacing(1)
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ")
        .block(Block::default().borders(Borders::ALL).title(" flameview "));

        let mut state = TableState::default();
        let selected = if self.rows.is_empty() {
            None
        } else {
            Some(self.selected.saturating_sub(self.scroll))
        };
        state.select(selected);
        f.render_stateful_widget(table, areas[1], &mut state);

        let footer_text = if self.show_help {
            "j/k move  h collapse  l/enter expand  / search  n/p match  [ ] min%  f hide-runtime  H hot-only  s sort  z auto-expand  a expand-visible  r reset  q quit"
                .to_string()
        } else {
            self.status_line()
        };
        f.render_widget(
            Paragraph::new(footer_text).wrap(Wrap { trim: false }),
            areas[2],
        );
    }

    pub fn on_key(&mut self, key: KeyCode) -> Action {
        match self.input_mode {
            InputMode::Search => self.on_search_key(key),
            InputMode::Normal => self.on_normal_key(key),
        }
    }

    #[cfg(test)]
    pub(crate) fn snapshot_text(&mut self, viewport_rows: usize, name_width: usize) -> String {
        self.viewport_rows = viewport_rows.max(1);
        self.rebuild_rows(self.viewport_rows);
        let (start, end) = self.table_window_bounds();

        let mut lines = Vec::new();
        lines.push(self.header_line());
        lines.push(format!(
            "{:>11} {:>6} {:>6} {:<10} {:<5} {}",
            "total", "tot%", "self%", "spark", "flag", "frame"
        ));
        for row in &self.rows[start..end] {
            let marker = if row.has_children {
                if row.expanded {
                    "v"
                } else {
                    ">"
                }
            } else {
                " "
            };
            let indent = "  ".repeat(row.depth.saturating_sub(1));
            let mut name = format!("{indent}{marker} {}", row.label);
            if name.len() > name_width {
                name.truncate(name_width.saturating_sub(1));
                name.push('~');
            }
            lines.push(format!(
                "{:>11} {:>5.1} {:>5.1} {:<10} {:<5} {}",
                format_count(row.total_count),
                row.total_pct,
                row.self_pct,
                spark_bar(row.total_pct, row.self_pct, 10),
                row.flags,
                name
            ));
        }
        lines.push(self.status_line());
        lines.join("\n")
    }

    #[cfg(test)]
    pub(crate) fn visible_rows_len(&mut self, viewport_rows: usize) -> usize {
        self.rebuild_rows(viewport_rows.max(1));
        self.rows.len()
    }

    fn on_search_key(&mut self, key: KeyCode) -> Action {
        match key {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.status_msg = "search cancelled".to_string();
            }
            KeyCode::Enter => {
                self.input_mode = InputMode::Normal;
                self.auto_expand_to_fill(self.viewport_rows);
                self.jump_to_first_match();
                self.status_msg = format!("search applied: '{}'", self.filters.query);
            }
            KeyCode::Backspace => {
                self.filters.query.pop();
                self.rebuild_rows(self.viewport_rows);
                self.jump_to_first_match();
            }
            KeyCode::Down => self.next_match(),
            KeyCode::Up => self.prev_match(),
            KeyCode::Char(c) => {
                self.filters.query.push(c);
                self.rebuild_rows(self.viewport_rows);
                self.jump_to_first_match();
            }
            _ => {}
        }
        Action::None
    }

    fn on_normal_key(&mut self, key: KeyCode) -> Action {
        match key {
            KeyCode::Char('q') | KeyCode::Char('Q') => return Action::Exit,
            KeyCode::Char('?') => {
                self.show_help = !self.show_help;
            }
            KeyCode::Down | KeyCode::Char('j') => self.move_selection(1),
            KeyCode::Up | KeyCode::Char('k') => self.move_selection(-1),
            KeyCode::PageDown => self.move_selection(self.viewport_rows as isize),
            KeyCode::PageUp => self.move_selection(-(self.viewport_rows as isize)),
            KeyCode::Home | KeyCode::Char('g') => self.select_index(0),
            KeyCode::End | KeyCode::Char('G') => {
                self.select_index(self.rows.len().saturating_sub(1));
            }
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Enter | KeyCode::Char('o') => {
                self.expand_selected();
            }
            KeyCode::Left | KeyCode::Char('h') => self.collapse_selected_or_parent(),
            KeyCode::Char('a') => self.expand_visible(),
            KeyCode::Char('z') => self.auto_expand_to_fill(self.viewport_rows),
            KeyCode::Char('/') => {
                self.input_mode = InputMode::Search;
                self.status_msg = "search mode".to_string();
            }
            KeyCode::Char('n') => self.next_match(),
            KeyCode::Char('p') => self.prev_match(),
            KeyCode::Char('f') => {
                self.filters.hide_runtime = !self.filters.hide_runtime;
                self.auto_expand_to_fill(self.viewport_rows);
                self.status_msg = format!("hide runtime = {}", self.filters.hide_runtime);
            }
            KeyCode::Char('H') => {
                self.filters.hot_only = !self.filters.hot_only;
                self.auto_expand_to_fill(self.viewport_rows);
                self.status_msg = format!("hot-only = {}", self.filters.hot_only);
            }
            KeyCode::Char('[') => {
                self.adjust_min_total(-0.5);
                self.auto_expand_to_fill(self.viewport_rows);
            }
            KeyCode::Char(']') => {
                self.adjust_min_total(0.5);
                self.auto_expand_to_fill(self.viewport_rows);
            }
            KeyCode::Char('s') => {
                self.sort_mode = self.sort_mode.next();
                self.auto_expand_to_fill(self.viewport_rows);
                self.status_msg = format!("sort = {}", self.sort_mode.label());
            }
            KeyCode::Char('r') => {
                self.filters = Filters::default();
                self.sort_mode = SortMode::TotalDesc;
                self.input_mode = InputMode::Normal;
                self.auto_expand_to_fill(self.viewport_rows);
                self.status_msg = "filters reset".to_string();
            }
            KeyCode::Char(' ') => {
                if self.row_selected().map(|r| r.expanded).unwrap_or(false) {
                    self.collapse_selected_or_parent();
                } else {
                    self.expand_selected();
                }
            }
            _ => {}
        }
        Action::None
    }

    fn header_line(&self) -> String {
        let visible = self.rows.len().saturating_sub(1);
        let mode = match self.input_mode {
            InputMode::Normal => "normal",
            InputMode::Search => "search",
        };
        format!(
            "flameview  visible:{visible}  samples:{}  mode:{mode}  sort:{}  q:'{}'  min:{:.1}%  hot:{}  hide-rt:{}",
            format_count(self.total_samples),
            self.sort_mode.label(),
            self.filters.query,
            self.filters.min_total_pct,
            self.filters.hot_only,
            self.filters.hide_runtime
        )
    }

    fn status_line(&self) -> String {
        if self.input_mode == InputMode::Search {
            format!(
                "search> {}  (type to filter, enter apply, esc cancel, j/k or n/p next match)",
                self.filters.query
            )
        } else if self.match_indices.is_empty() {
            format!(
                "{} | rows:{}/{}",
                self.status_msg,
                self.selected.saturating_add(1),
                self.rows.len()
            )
        } else {
            format!(
                "{} | match:{}/{} | rows:{}/{}",
                self.status_msg,
                self.match_cursor.saturating_add(1),
                self.match_indices.len(),
                self.selected.saturating_add(1),
                self.rows.len()
            )
        }
    }

    fn render_table_row(&self, row: &RowMetrics) -> Row<'static> {
        let marker = if row.has_children {
            if row.expanded {
                "v"
            } else {
                ">"
            }
        } else {
            " "
        };
        let indent = "  ".repeat(row.depth.saturating_sub(1));
        let frame = format!("{indent}{marker} {}", row.label);
        Row::new([
            Cell::from(format!("{:>11}", format_count(row.total_count))),
            Cell::from(format!("{:>5.1}", row.total_pct)),
            Cell::from(format!("{:>5.1}", row.self_pct)),
            Cell::from(format!(
                "{:<10}",
                spark_bar(row.total_pct, row.self_pct, 10)
            )),
            Cell::from(format!("{:<5}", row.flags)),
            Cell::from(frame),
        ])
    }

    fn move_selection(&mut self, delta: isize) {
        if self.rows.is_empty() {
            return;
        }
        let max = self.rows.len().saturating_sub(1) as isize;
        let next = (self.selected as isize + delta).clamp(0, max) as usize;
        self.select_index(next);
    }

    fn select_index(&mut self, idx: usize) {
        self.selected = idx.min(self.rows.len().saturating_sub(1));
        self.ensure_selection_visible();
    }

    fn row_selected(&self) -> Option<&RowMetrics> {
        self.rows.get(self.selected)
    }

    fn expand_selected(&mut self) {
        let Some(row) = self.row_selected() else {
            return;
        };
        if !row.has_children {
            return;
        }
        let id = row.id;
        let label = row.label.clone();
        self.expanded.insert(id);
        self.rebuild_rows(self.viewport_rows);
        self.status_msg = format!("expanded {label}");
    }

    fn collapse_selected_or_parent(&mut self) {
        let Some(row) = self.row_selected().cloned() else {
            return;
        };
        let root = self.tree.root();
        if row.id != root && row.expanded {
            self.expanded.remove(&row.id);
            self.rebuild_rows(self.viewport_rows);
            self.status_msg = format!("collapsed {}", row.label);
            return;
        }
        if let Some(parent) = self.parent_by_id.get(&row.id).copied() {
            self.select_node(parent);
            self.status_msg = "moved to parent".to_string();
        }
    }

    fn expand_visible(&mut self) {
        let ids: Vec<NodeId> = self
            .rows
            .iter()
            .filter(|row| row.has_children)
            .map(|row| row.id)
            .collect();
        for id in ids {
            self.expanded.insert(id);
        }
        self.rebuild_rows(self.viewport_rows);
        self.status_msg = "expanded visible branches".to_string();
    }

    fn adjust_min_total(&mut self, delta: f64) {
        self.filters.min_total_pct = (self.filters.min_total_pct + delta).clamp(0.0, 100.0);
        self.status_msg = format!("min total% = {:.1}", self.filters.min_total_pct);
    }

    fn rebuild_rows(&mut self, viewport_rows: usize) {
        self.viewport_rows = viewport_rows.max(1);
        self.rows.clear();
        let mut subtree_match_memo = HashMap::new();
        self.collect_rows(self.tree.root(), 0, &mut subtree_match_memo);
        if self.rows.is_empty() {
            self.selected = 0;
            self.scroll = 0;
        } else {
            self.selected = self.selected.min(self.rows.len() - 1);
        }
        self.update_match_indices();
        self.ensure_selection_visible();
    }

    fn collect_rows(
        &mut self,
        id: NodeId,
        depth: usize,
        subtree_match_memo: &mut HashMap<NodeId, bool>,
    ) {
        let root = self.tree.root();
        let subtree_matches = self.subtree_matches(id, subtree_match_memo);
        if id != root && !subtree_matches {
            return;
        }

        let node = &self.tree[id];
        let has_children = node.first_child.is_some();
        let child_count = self.child_count(id);
        let total_pct = ratio_pct(node.total_count, self.total_samples);
        let self_pct = ratio_pct(node.self_count, self.total_samples);
        let hide_runtime_bridge =
            self.filters.hide_runtime && id != root && is_runtime_frame(&node.name);
        let expanded = id == root || self.expanded.contains(&id);

        if !hide_runtime_bridge {
            self.rows.push(RowMetrics {
                id,
                depth,
                label: node.name.clone(),
                total_count: node.total_count,
                self_count: node.self_count,
                child_count,
                total_pct,
                self_pct,
                has_children,
                expanded,
                flags: heuristic_flags(has_children, child_count, total_pct, self_pct),
            });
        }

        if !hide_runtime_bridge && !expanded {
            return;
        }

        let child_depth = if hide_runtime_bridge {
            depth
        } else {
            depth.saturating_add(1)
        };
        let mut children = self.children(id);
        self.sort_children(&mut children);
        for child in children {
            self.collect_rows(child, child_depth, subtree_match_memo);
        }
    }

    fn subtree_matches(&self, id: NodeId, memo: &mut HashMap<NodeId, bool>) -> bool {
        if let Some(v) = memo.get(&id).copied() {
            return v;
        }
        let mut matched = if id == self.tree.root() {
            true
        } else {
            self.node_matches_filters(id)
        };

        for child in self.children(id) {
            if self.subtree_matches(child, memo) {
                matched = true;
            }
        }
        memo.insert(id, matched);
        matched
    }

    fn node_matches_filters(&self, id: NodeId) -> bool {
        let node = &self.tree[id];
        let total_pct = ratio_pct(node.total_count, self.total_samples);
        let self_pct = ratio_pct(node.self_count, self.total_samples);

        if total_pct < self.filters.min_total_pct {
            return false;
        }
        if self.filters.hot_only && self_pct < HOT_SELF_PCT {
            return false;
        }
        if self.filters.hide_runtime && is_runtime_frame(&node.name) {
            return false;
        }
        if !self.filters.query.is_empty() {
            let q = self.filters.query.to_lowercase();
            if !node.name.to_lowercase().contains(&q) {
                return false;
            }
        }
        true
    }

    fn children(&self, id: NodeId) -> Vec<NodeId> {
        let mut out = Vec::new();
        let mut child = self.tree[id].first_child;
        while let Some(cid) = child {
            out.push(cid);
            child = self.tree[cid].next_sibling;
        }
        out
    }

    fn child_count(&self, id: NodeId) -> usize {
        let mut count = 0;
        let mut child = self.tree[id].first_child;
        while let Some(cid) = child {
            count += 1;
            child = self.tree[cid].next_sibling;
        }
        count
    }

    fn sort_children(&self, ids: &mut [NodeId]) {
        match self.sort_mode {
            SortMode::TotalDesc => ids.sort_by(|a, b| {
                self.tree[*b]
                    .total_count
                    .cmp(&self.tree[*a].total_count)
                    .then_with(|| self.tree[*b].self_count.cmp(&self.tree[*a].self_count))
            }),
            SortMode::SelfDesc => ids.sort_by(|a, b| {
                self.tree[*b]
                    .self_count
                    .cmp(&self.tree[*a].self_count)
                    .then_with(|| self.tree[*b].total_count.cmp(&self.tree[*a].total_count))
            }),
            SortMode::NameAsc => ids.sort_by(|a, b| self.tree[*a].name.cmp(&self.tree[*b].name)),
        }
    }

    fn auto_expand_to_fill(&mut self, viewport_rows: usize) {
        let target = viewport_rows.max(1);
        let root = self.tree.root();
        self.expanded.insert(root);

        let expansion_budget = target * 4 + 32;
        let mut expanded_steps = 0usize;
        loop {
            self.rebuild_rows(target);
            if self.rows.len() >= target || expanded_steps >= expansion_budget {
                break;
            }

            let candidate = self
                .rows
                .iter()
                .filter(|row| row.has_children && !row.expanded)
                .max_by(|a, b| {
                    let a_is_chain = a.child_count == 1 && a.self_count == 0;
                    let b_is_chain = b.child_count == 1 && b.self_count == 0;
                    let a_not_runtime = !is_runtime_frame(&a.label);
                    let b_not_runtime = !is_runtime_frame(&b.label);

                    (
                        a_not_runtime as u8,
                        (!a_is_chain) as u8,
                        a.self_count,
                        a.total_count,
                        usize::MAX - a.depth,
                    )
                        .cmp(&(
                            b_not_runtime as u8,
                            (!b_is_chain) as u8,
                            b.self_count,
                            b.total_count,
                            usize::MAX - b.depth,
                        ))
                })
                .map(|row| row.id);

            let Some(id) = candidate else {
                break;
            };
            if !self.expanded.insert(id) {
                break;
            }
            expanded_steps += 1;
        }
        self.rebuild_rows(target);
    }

    fn ensure_selection_visible(&mut self) {
        if self.rows.is_empty() {
            self.selected = 0;
            self.scroll = 0;
            return;
        }

        let height = self.viewport_rows.max(1);
        if self.selected < self.scroll {
            self.scroll = self.selected;
        } else if self.selected >= self.scroll + height {
            self.scroll = self.selected + 1 - height;
        }
        let max_scroll = self.rows.len().saturating_sub(height);
        self.scroll = self.scroll.min(max_scroll);
    }

    fn table_window_bounds(&mut self) -> (usize, usize) {
        self.ensure_selection_visible();
        let start = self.scroll.min(self.rows.len());
        let end = (start + self.viewport_rows).min(self.rows.len());
        (start, end)
    }

    fn update_match_indices(&mut self) {
        if self.filters.query.is_empty() {
            self.match_indices.clear();
            self.match_cursor = 0;
            return;
        }
        let q = self.filters.query.to_lowercase();
        self.match_indices = self
            .rows
            .iter()
            .enumerate()
            .filter_map(|(idx, row)| {
                if row.label.to_lowercase().contains(&q) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();
        if self.match_indices.is_empty() {
            self.match_cursor = 0;
            return;
        }
        if self.match_cursor >= self.match_indices.len() {
            self.match_cursor = 0;
        }
    }

    fn jump_to_first_match(&mut self) {
        if let Some(first) = self.match_indices.first().copied() {
            self.select_index(first);
            self.match_cursor = 0;
        }
    }

    fn next_match(&mut self) {
        if self.match_indices.is_empty() {
            self.status_msg = "no matches".to_string();
            return;
        }
        self.match_cursor = (self.match_cursor + 1) % self.match_indices.len();
        let idx = self.match_indices[self.match_cursor];
        self.select_index(idx);
        self.status_msg = format!(
            "match {}/{}",
            self.match_cursor + 1,
            self.match_indices.len()
        );
    }

    fn prev_match(&mut self) {
        if self.match_indices.is_empty() {
            self.status_msg = "no matches".to_string();
            return;
        }
        self.match_cursor = if self.match_cursor == 0 {
            self.match_indices.len() - 1
        } else {
            self.match_cursor - 1
        };
        let idx = self.match_indices[self.match_cursor];
        self.select_index(idx);
        self.status_msg = format!(
            "match {}/{}",
            self.match_cursor + 1,
            self.match_indices.len()
        );
    }

    fn select_node(&mut self, id: NodeId) {
        if let Some(idx) = self.rows.iter().position(|row| row.id == id) {
            self.select_index(idx);
        }
    }
}

fn build_parent_map(tree: &FlameTree) -> HashMap<NodeId, NodeId> {
    let mut parents = HashMap::new();
    let root = tree.root();
    parents.insert(root, root);
    let mut stack = vec![root];

    while let Some(parent) = stack.pop() {
        let mut child = tree[parent].first_child;
        while let Some(cid) = child {
            parents.insert(cid, parent);
            stack.push(cid);
            child = tree[cid].next_sibling;
        }
    }
    parents
}

fn ratio_pct(numerator: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        numerator as f64 / total as f64 * 100.0
    }
}

fn format_count(n: u64) -> String {
    let units = ["", "K", "M", "G", "T", "P"];
    let mut value = n as f64;
    let mut idx = 0usize;
    while value >= 1024.0 && idx < units.len() - 1 {
        value /= 1024.0;
        idx += 1;
    }
    if idx == 0 {
        format!("{n}")
    } else {
        format!("{value:.1}{}", units[idx])
    }
}

fn spark_bar(total_pct: f64, self_pct: f64, width: usize) -> String {
    if width == 0 {
        return String::new();
    }
    let total_cells = ((total_pct.clamp(0.0, 100.0) / 100.0) * width as f64).round() as usize;
    let mut self_cells = ((self_pct.clamp(0.0, 100.0) / 100.0) * width as f64).round() as usize;
    let total_cells = total_cells.min(width);
    self_cells = self_cells.min(total_cells);

    let mut out = String::with_capacity(width);
    out.push_str(&"=".repeat(self_cells));
    out.push_str(&"#".repeat(total_cells.saturating_sub(self_cells)));
    out.push_str(&".".repeat(width.saturating_sub(total_cells)));
    out
}

fn heuristic_flags(
    has_children: bool,
    child_count: usize,
    total_pct: f64,
    self_pct: f64,
) -> String {
    let mut flags = String::new();
    if !has_children && self_pct >= 2.0 {
        flags.push('L'); // hot leaf
    }
    if child_count >= 6 && total_pct >= 2.0 {
        flags.push('F'); // fanout
    }
    if child_count == 1 && total_pct >= 5.0 {
        flags.push('C'); // chain
    }
    if self_pct >= 5.0 {
        flags.push('S'); // high self
    }
    if flags.is_empty() {
        "-".to_string()
    } else {
        flags
    }
}

fn is_runtime_frame(name: &str) -> bool {
    let mut normalized = name.trim_start_matches(|c: char| {
        matches!(c, '<' | '>' | '&' | '*' | '(' | ')' | '\'' | ' ')
    });
    if let Some(stripped) = normalized.strip_prefix("dyn ") {
        normalized = stripped;
    }

    const PREFIXES: &[&str] = &[
        "std::",
        "core::",
        "alloc::",
        "criterion::",
        "test::",
        "__libc",
        "_start",
        "main",
        "[libc",
        "[ld-linux",
    ];
    PREFIXES
        .iter()
        .any(|prefix| normalized.starts_with(prefix))
}

#[cfg(all(test, not(miri)))]
mod tests {
    use std::io::Cursor;

    use flameview::loader::collapsed;
    use insta::assert_snapshot;
    use ratatui::crossterm::event::KeyCode;

    use super::App;

    fn sample_tree() -> flameview::arena::FlameTree {
        let data = "\
root;frontend;render;diff 90
root;frontend;render;layout 35
root;frontend;events;click 22
root;frontend;events;mousemove 10
root;backend;db;query 80
root;backend;db;query 30
root;backend;cache;lookup 20
root;backend;api;serialize 18
root;backend;api;serialize 8
root;backend;api;parse 7
";
        collapsed::load_stream(Cursor::new(data.as_bytes())).unwrap()
    }

    fn runtime_chain_tree() -> flameview::arena::FlameTree {
        let data = "\
root;streaming_json_;_start;__libc_start_main;main;std::rt::lang_start_internal;std::sys::backtrace::__rust_begin_short_backtrace;streaming_json_large::main;streaming_json_large::benches;parse_values 900
root;streaming_json_;_start;__libc_start_main;main;std::rt::lang_start_internal;std::sys::backtrace::__rust_begin_short_backtrace;streaming_json_large::main;streaming_json_large::benches;parse_events 600
root;streaming_json_;_start;__libc_start_main;main;std::rt::lang_start_internal;std::sys::backtrace::__rust_begin_short_backtrace;streaming_json_large::main;streaming_json_large::cleanup 30
";
        collapsed::load_stream(Cursor::new(data.as_bytes())).unwrap()
    }

    #[test]
    fn snapshot_initial_layout() {
        let tree = sample_tree();
        let mut app = App::new(&tree);
        let snapshot = app.snapshot_text(14, 48);
        assert_snapshot!("viewer_initial_layout", snapshot);
    }

    #[test]
    fn snapshot_after_key_sequence() {
        let tree = sample_tree();
        let mut app = App::new(&tree);
        app.on_key(KeyCode::Char(']'));
        app.on_key(KeyCode::Char('H'));
        app.on_key(KeyCode::Char('/'));
        for ch in "query".chars() {
            app.on_key(KeyCode::Char(ch));
        }
        app.on_key(KeyCode::Enter);
        let snapshot = app.snapshot_text(14, 48);
        assert_snapshot!("viewer_filtered_sequence", snapshot);
    }

    #[test]
    fn auto_expand_fills_rows() {
        let tree = sample_tree();
        let mut app = App::new(&tree);
        let visible = app.visible_rows_len(10);
        assert!(visible >= 8, "expected auto expansion, got {visible}");
    }

    #[test]
    fn runtime_bridges_hidden_by_default() {
        let tree = runtime_chain_tree();
        let mut app = App::new(&tree);
        let snapshot = app.snapshot_text(14, 56);
        assert!(snapshot.contains("hide-rt:true"), "{snapshot}");
        assert!(!snapshot.contains("_start"), "{snapshot}");
        assert!(!snapshot.contains("std::rt::lang_start_internal"), "{snapshot}");
        assert!(snapshot.contains("streaming_json_large::benches"), "{snapshot}");
    }

    #[test]
    fn runtime_filter_toggle_reveals_runtime_frames() {
        let tree = runtime_chain_tree();
        let mut app = App::new(&tree);
        app.on_key(KeyCode::Char('f'));
        let snapshot = app.snapshot_text(14, 56);
        assert!(snapshot.contains("hide-rt:false"), "{snapshot}");
        assert!(snapshot.contains("_start"), "{snapshot}");
        assert!(snapshot.contains("std::rt::lang_start_internal"), "{snapshot}");
    }
}
