#![cfg(not(miri))]

use assert_cmd::prelude::*;
use std::path::PathBuf;
use std::process::Command;

#[allow(dead_code)]
#[path = "../src/build.rs"]
mod build;
#[path = "support/mock_exec.rs"]
mod mock_exec;
#[path = "../src/cli/opts.rs"]
mod opts;
use build::{find_crate_root, find_unique_target, Artifact};
use mock_exec::{success, MockCommandExecutor};
use opts::{Opt, TargetKind};
use std::fs;
use tempfile::tempdir;

fn artifact_example(path: &str) -> Artifact {
    serde_json::from_value(serde_json::json!({
        "package_id": "pkg",
        "manifest_path": "",
        "target": {
            "name": "eg",
            "kind": ["bin"],
            "crate_types": [],
            "required-features": [],
            "src_path": "src/main.rs",
            "edition": "2021",
            "doctest": true,
            "test": true,
            "doc": true
        },
        "profile": {
            "opt_level": "0",
            "debuginfo": 0,
            "debug_assertions": true,
            "overflow_checks": true,
            "test": false
        },
        "features": [],
        "filenames": [],
        "executable": path,
        "fresh": true
    }))
    .unwrap()
}

#[test]
fn find_crate_root_finds_manifest() {
    let tmp = tempdir().unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "").unwrap();
    let nested = tmp.path().join("a/b");
    fs::create_dir_all(&nested).unwrap();
    let old = std::env::current_dir().unwrap();
    std::env::set_current_dir(&nested).unwrap();
    let root = find_crate_root(None).unwrap();
    assert_eq!(root, tmp.path().canonicalize().unwrap());
    std::env::set_current_dir(old).unwrap();
}

#[test]
fn find_unique_target_errors_on_ambiguous() {
    let tmp = tempdir().unwrap();
    fs::write(
        tmp.path().join("Cargo.toml"),
        r#"[package]
name="double"
version="0.1.0"
edition="2021"
[[bin]]
name="one"
path="one.rs"
[[bin]]
name="two"
path="two.rs"
"#,
    )
    .unwrap();
    fs::write(tmp.path().join("one.rs"), "fn main() {}").unwrap();
    fs::write(tmp.path().join("two.rs"), "fn main() {}").unwrap();
    let manifest = tmp.path().join("Cargo.toml");
    let err =
        find_unique_target(&[TargetKind::Bin], None, Some(manifest.as_path()), None).unwrap_err();
    assert!(err.to_string().contains("multiple targets"));
}

#[test]
fn workload_warns_without_debuginfo() {
    let crate_dir: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "fixtures",
        "simple-example",
    ]
    .iter()
    .collect();
    let manifest_cli: PathBuf = [env!("CARGO_MANIFEST_DIR"), "Cargo.toml"].iter().collect();
    let assert = Command::new("cargo")
        .current_dir(&crate_dir)
        .args([
            "run",
            "--manifest-path",
            manifest_cli.to_str().unwrap(),
            "--bin",
            "cargo-flameview",
            "--",
            "--release",
            "--example",
            "eg",
            "--",
            "--help",
        ])
        .assert()
        .success();
    let stderr = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
    assert!(stderr.contains("debuginfo"));
}

#[test]
fn build_and_select_example_binary() {
    let crate_dir: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "fixtures",
        "simple-example",
    ]
    .iter()
    .collect();
    let manifest_cli: PathBuf = [env!("CARGO_MANIFEST_DIR"), "Cargo.toml"].iter().collect();
    let assert = Command::new("cargo")
        .current_dir(&crate_dir)
        .args([
            "run",
            "--manifest-path",
            manifest_cli.to_str().unwrap(),
            "--bin",
            "cargo-flameview",
            "--",
            "--example",
            "eg",
            "--",
            "--help",
        ])
        .assert()
        .success();
    let out = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let line = out.lines().next().unwrap();
    assert!(line.ends_with("target/debug/examples/eg --help"));
}

#[test]
fn build_uses_mock_executor() {
    use cargo_metadata::Message;
    use std::process::Output;

    let artifact = artifact_example("/tmp/eg");
    let msg = Message::CompilerArtifact(artifact.clone());
    let mut stdout = serde_json::to_vec(&msg).unwrap();
    stdout.push(b'\n');
    let output = Output {
        status: success(),
        stdout,
        stderr: Vec::new(),
    };
    let expected = vec![vec![
        "cargo".into(),
        "build".into(),
        "--bin".into(),
        "eg".into(),
        "--message-format=json-render-diagnostics".into(),
    ]];
    let script = expected.into_iter().zip(std::iter::once(output)).collect();
    let exec = MockCommandExecutor::new(script);
    let opt = Opt {
        dev: false,
        profile: None,
        package: None,
        bin: Some("eg".into()),
        example: None,
        test: None,
        bench: None,
        unit_test: None,
        unit_bench: None,
        unit_test_kind: None,
        manifest_path: None,
        target: None,
        features: None,
        no_default_features: false,
        release: false,
        trailing_arguments: vec![],
    };
    let artifacts = build::build(&exec, &opt, vec![TargetKind::Bin]).unwrap();
    assert_eq!(artifacts.len(), 1);
    assert_eq!(artifacts[0].target.name, "eg");
}
