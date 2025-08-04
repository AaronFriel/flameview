#![cfg(not(miri))]

use std::path::PathBuf;

use flameview_cli::{viewer::run_with_backend, ViewArgs};
use ratatui::backend::TestBackend;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

#[test]
#[should_panic]
fn tui_wrong_frame_len_panics() {
    let data_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "..",
        "..",
        "tests",
        "data",
        "small.txt",
    ]
    .iter()
    .collect();
    let args = ViewArgs {
        file: data_path,
        summarize: false,
        max_lines: 50,
        coverage: 0.95,
    };
    let backend = TestBackend::new(80, 24);
    let script = vec![
        KeyEvent::from(KeyCode::Down),
        KeyEvent::from(KeyCode::Down),
        KeyEvent::from(KeyCode::Right),
        KeyEvent::from(KeyCode::Char('h')),
    ];
    let frames = run_with_backend(&args, backend, script).expect("run");
    assert_eq!(frames.len(), 5);
}
