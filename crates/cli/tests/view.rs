#![cfg(not(miri))]

use std::path::PathBuf;
use std::process::Command;

use flameview_cli::{viewer, ViewArgs};

#[test]
fn viewer_loads_tree() {
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
    viewer::tui(&args).expect("tui runs");
}

#[test]
fn viewer_skips_when_not_tty() {
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
    viewer::tui(&args).expect("should fall back to summary mode");
}

#[test]
fn cargo_flameview_detects_folded() {
    let cargo_exe = env!("CARGO_BIN_EXE_cargo-flameview");
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

    let output = Command::new(cargo_exe)
        .arg(&data_path)
        .output()
        .expect("run");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("(root)"));
}
