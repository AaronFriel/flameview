#![cfg(not(miri))]

use std::path::PathBuf;
use std::process::Command;

#[test]
fn cli_summarize_runs() {
    let exe = env!("CARGO_BIN_EXE_flameview");
    let data_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "..",
        "..",
        "tests",
        "data",
        "perl.txt",
    ]
    .iter()
    .collect();
    let out = Command::new(exe)
        .arg(data_path)
        .arg("--summarize")
        .arg("--max-lines")
        .arg("20")
        .arg("--coverage")
        .arg("0.8")
        .output()
        .expect("run");
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    insta::assert_snapshot!(stdout, @r"
    (root) (100.0%, 1000)
      foo (60.0%, 600)
        bar (40.0%, 400)
      baz (35.0%, 350)
    ");
    assert!(stdout.contains("(root)"));
    assert!(stdout.lines().count() <= 21);
}

#[test]
fn cli_summarize_stdin() {
    use std::io::Write;
    let exe = env!("CARGO_BIN_EXE_flameview");
    let data_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "..",
        "..",
        "tests",
        "data",
        "perl.txt",
    ]
    .iter()
    .collect();
    let data = std::fs::read(&data_path).unwrap();
    let mut child = Command::new(exe)
        .arg("-")
        .arg("--summarize")
        .arg("--max-lines")
        .arg("20")
        .arg("--coverage")
        .arg("0.8")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("spawn");
    child.stdin.as_mut().unwrap().write_all(&data).unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("(root)"));
    assert!(stdout.lines().count() <= 21);
}
