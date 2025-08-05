use ratatui::crossterm::event::KeyCode;
use ratatui::prelude::*;

use flameview::arena::FlameTree;

use super::spark_tree_view::SparkTreeView;

pub enum Action {
    None,
    Exit,
}

pub struct App<'a> {
    view: SparkTreeView<'a>,
}

impl<'a> App<'a> {
    pub fn new(tree: &'a FlameTree, max_lines: usize, coverage: f64) -> Self {
        Self {
            view: SparkTreeView::new(tree, max_lines, coverage),
        }
    }

    pub fn draw(&mut self, f: &mut Frame) {
        self.view.draw(f);
    }

    pub fn on_key(&mut self, key: KeyCode) -> Action {
        match key {
            KeyCode::Char('q') | KeyCode::Char('Q') => Action::Exit,
            KeyCode::Down => {
                self.view.down();
                Action::None
            }
            KeyCode::Up => {
                self.view.up();
                Action::None
            }
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Enter => {
                self.view.expand();
                Action::None
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Backspace => {
                self.view.collapse();
                Action::None
            }
            KeyCode::Char('s') => {
                self.view.toggle_spark();
                Action::None
            }
            _ => Action::None,
        }
    }
}
