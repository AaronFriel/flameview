#![cfg(all(not(windows), not(miri)))]

use std::path::PathBuf;

use flameview_cli::{viewer::run_with_backend, ViewArgs};
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

#[test]
fn tui_navigation_snapshot() {
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
        KeyEvent::from(KeyCode::Char('q')),
    ];
    let frames = run_with_backend(&args, backend, script).expect("run");
    fn buf_to_string(b: &Buffer) -> String {
        let mut out = String::new();
        for y in 0..b.area.height {
            let mut line = String::new();
            for x in 0..b.area.width {
                line.push_str(b[(x, y)].symbol());
            }
            while line.ends_with(' ') {
                line.pop();
            }
            out.push_str(&line);
            if y + 1 < b.area.height {
                out.push('\n');
            }
        }
        out
    }
    let ascii = frames
        .iter()
        .map(buf_to_string)
        .collect::<Vec<_>>()
        .join("\n--- frame ---\n");
    insta::with_settings!({ prepend_module_to_snapshot => false }, {
        insta::assert_snapshot!("tui_navigation", ascii);
    });
}
