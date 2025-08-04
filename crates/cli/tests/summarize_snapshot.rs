#![cfg(not(miri))]

use std::path::{Path, PathBuf};
use std::process::Command;

use regex::Regex;

fn run_summary(fixture: &Path, max_lines: usize, coverage: f64) -> String {
    let bin = env!("CARGO_BIN_EXE_flameview");
    let output = Command::new(bin)
        .arg(fixture)
        .arg("--summarize")
        .arg("--max-lines")
        .arg(max_lines.to_string())
        .arg("--coverage")
        .arg(coverage.to_string())
        .output()
        .expect("run");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    let re = Regex::new(r"[A-Z]:[/\\][^\s]*").expect("regex");
    let cleaned = re.replace_all(&stdout, "");
    let cleaned = cleaned.replace("\r\n", "\n");
    let lines: Vec<_> = cleaned.lines().map(|l| l.trim_end()).collect();
    lines.join("\n").trim_end_matches('\n').to_string()
}

#[test]
fn summarize_snapshots() {
    let data_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "..", "..", "tests", "data"]
        .into_iter()
        .collect();

    let cases = [
        ("small_default", data_dir.join("small.txt"), 50, 0.95),
        ("medium_30_90", data_dir.join("medium.txt"), 30, 0.90),
        ("large_15_80", data_dir.join("large.txt"), 15, 0.80),
    ];

    for (name, fixture, lines, cov) in cases {
        let output = run_summary(&fixture, lines, cov);
        let mut settings = insta::Settings::new();
        settings.set_prepend_module_to_snapshot(false);
        settings.bind(|| insta::assert_snapshot!(name, output));
    }
}

#[test]
#[should_panic]
fn summarize_corrupt_panics() {
    let bad: PathBuf = [env!("CARGO_MANIFEST_DIR"), "..", "..", "README.md"]
        .into_iter()
        .collect();

    run_summary(&bad, 10, 0.5);
}
