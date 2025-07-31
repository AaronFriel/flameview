use std::path::PathBuf;
use std::process::Command;

#[test]
fn cli_summarize_runs() {
    let exe = env!("CARGO_BIN_EXE_flameview-cli");
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
        .arg("summarize")
        .arg(data_path)
        .arg("--max-lines")
        .arg("20")
        .arg("--coverage")
        .arg("0.8")
        .output()
        .expect("run");
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("(root)"));
    assert!(stdout.lines().count() <= 21);
}
