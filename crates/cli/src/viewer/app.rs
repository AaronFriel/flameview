use ratatui::crossterm::event::KeyCode;
use ratatui::prelude::*;
use tui_tree_widget::{Tree, TreeItem, TreeState};

use flameview::arena::{FlameTree, NodeId};

pub enum Action {
    None,
    Exit,
    ToggleHelp,
}

pub struct App {
    items: Vec<TreeItem<'static, String>>,
    state: TreeState<String>,
    #[allow(dead_code)]
    show_help: bool,
}

impl App {
    pub fn new(tree: &FlameTree) -> Self {
        let items = vec![to_tree_item(tree, tree.root())];
        let mut state = TreeState::default();
        state.open(vec!["root".to_string()]);
        Self {
            items,
            state,
            show_help: false,
        }
    }

    pub fn draw(&mut self, f: &mut Frame) {
        let widget = Tree::new(&self.items).expect("tree");
        f.render_stateful_widget(widget, f.area(), &mut self.state);
    }

    pub fn on_key(&mut self, key: KeyCode) -> Action {
        match key {
            KeyCode::Char('q') | KeyCode::Char('Q') => Action::Exit,
            KeyCode::Char('h') => {
                self.show_help = !self.show_help;
                Action::ToggleHelp
            }
            KeyCode::Left => {
                self.state.key_left();
                Action::None
            }
            KeyCode::Right => {
                self.state.key_right();
                Action::None
            }
            KeyCode::Up => {
                self.state.key_up();
                Action::None
            }
            KeyCode::Down => {
                self.state.key_down();
                Action::None
            }
            _ => Action::None,
        }
    }
}

fn to_tree_item(tree: &FlameTree, id: NodeId) -> TreeItem<'static, String> {
    let node = &tree[id];
    let mut children = Vec::new();
    let mut child = node.first_child;
    while let Some(cid) = child {
        children.push(to_tree_item(tree, cid));
        child = tree[cid].next_sibling;
    }
    let label = format!("{} ({})", node.name, node.total_count);
    if children.is_empty() {
        TreeItem::new_leaf(node.name.clone(), label)
    } else {
        TreeItem::new(node.name.clone(), label, children).expect("children unique")
    }
}
