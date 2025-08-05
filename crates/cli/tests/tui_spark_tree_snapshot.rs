#![cfg(all(not(windows), not(miri)))]

use std::path::PathBuf;

use flameview_cli::{viewer::run_with_backend, ViewArgs};
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

#[test]
fn tui_spark_tree_snapshot() {
    let data_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "..",
        "..",
        "tests",
        "data",
        "small_self.txt",
    ]
    .iter()
    .collect();
    let args = ViewArgs {
        file: data_path,
        summarize: false,
        max_lines: 4,
        coverage: 0.9,
    };
    let backend = TestBackend::new(80, 12);
    let script = vec![
        KeyEvent::from(KeyCode::Char('s')),
        KeyEvent::from(KeyCode::Down),
        KeyEvent::from(KeyCode::Down),
        KeyEvent::from(KeyCode::Down),
        KeyEvent::from(KeyCode::Down),
        KeyEvent::from(KeyCode::Right),
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
    let indices = [0usize, 1, 6];
    let ascii = indices
        .iter()
        .map(|&i| buf_to_string(&frames[i]))
        .collect::<Vec<_>>()
        .join("\n--- frame ---\n");
    insta::with_settings!({ prepend_module_to_snapshot => false }, {
        insta::assert_snapshot!("tui_spark_tree", ascii);
    });
}
