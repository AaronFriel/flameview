#![cfg(not(miri))]

use std::{fs, path::PathBuf, process::Command};

#[test]
fn cargo_flamegraph_saves_folded_stacks() {
    // locate fixture crate
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("tests")
        .join("flamegraph_fixture")
        .join("Cargo.toml");

    let temp = tempfile::tempdir().unwrap();
    let svg = temp.path().join("graph.svg");
    let folded = temp.path().join("stacks.folded");

    let mut cmd = Command::new("cargo");
    cmd.args([
        "flamegraph",
        "--manifest-path",
        manifest.to_str().unwrap(),
        "--bin",
        "flamegraph-fixture",
        "--freq",
        "100",
        "--output",
        svg.to_str().unwrap(),
        "--post-process",
        &format!("tee {}", folded.to_string_lossy()),
    ]);

    if let Ok(entries) = std::fs::read_dir("/usr/lib") {
        for e in entries.flatten() {
            if e.file_name().to_string_lossy().starts_with("linux-tools-") {
                let path = e.path().join("perf");
                if path.exists() {
                    cmd.env("PERF", path);
                    break;
                }
            }
        }
    }

    let status = cmd.status().expect("run cargo flamegraph");

    assert!(status.success());
    let data = fs::read_to_string(&folded).expect("read folded stacks");
    assert!(!data.trim().is_empty());
    assert!(data.lines().any(|l| l.contains(';')));
}
