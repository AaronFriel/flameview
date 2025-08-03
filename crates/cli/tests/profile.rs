#![cfg(not(miri))]

use assert_cmd::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

#[allow(dead_code)]
#[path = "../src/build.rs"]
mod build;
#[path = "../src/cli/opts.rs"]
mod opts;
#[allow(dead_code)]
#[path = "../src/profile.rs"]
mod profile;

use build::CommandExecutor;
use opts::Opt;
use profile::{profile, ProfileOptions};

struct MockPerf;

impl CommandExecutor for MockPerf {
    fn run(&self, cmd: &mut Command) -> std::io::Result<Output> {
        let program = cmd.get_program().to_string_lossy().into_owned();
        let args = cmd
            .get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        if program == "perf" && args.first().map(|s| s.as_str()) == Some("record") {
            assert_eq!(args.last().unwrap(), "/bin/true");
            Ok(Output {
                status: mock_exec::success(),
                stdout: Vec::new(),
                stderr: Vec::new(),
            })
        } else if program == "perf" && args.first().map(|s| s.as_str()) == Some("script") {
            Ok(Output {
                status: mock_exec::success(),
                stdout: b"foo 1/1 0: cpu-clock:\n    0 main (foo)\n\n".to_vec(),
                stderr: Vec::new(),
            })
        } else {
            panic!("unexpected command: {program} {args:?}");
        }
    }
}

#[allow(dead_code)]
#[path = "support/mock_exec.rs"]
mod mock_exec;

fn default_opt() -> Opt {
    Opt {
        dev: false,
        cargo_profile: None,
        package: None,
        bin: None,
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
        profile: ProfileOptions {
            frequency: 99,
            output: None,
            keep_perf_data: false,
            cmd: None,
        },
        trailing_arguments: vec![],
    }
}

#[test]
fn profile_creates_folded_file() {
    let exec = MockPerf;
    let opt = default_opt();
    let result = profile(&exec, &["/bin/true".into()], &opt).unwrap();
    let data = fs::read_to_string(&result.folded_path).unwrap();
    assert!(!data.is_empty());
}

#[test]
fn profile_respects_output_override() {
    let exec = MockPerf;
    let mut opt = default_opt();
    let dir = tempfile::tempdir().unwrap();
    let custom = dir.path().join("custom.folded");
    opt.profile.output = Some(custom.clone());
    let result = profile(&exec, &["/bin/true".into()], &opt).unwrap();
    assert_eq!(result.folded_path, custom);
}

#[test]
fn run_profile_on_fixture() {
    let crate_dir: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "fixtures",
        "simple-example",
    ]
    .iter()
    .collect();
    let manifest_cli: PathBuf = [env!("CARGO_MANIFEST_DIR"), "Cargo.toml"].iter().collect();
    Command::new("cargo")
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
        ])
        .assert()
        .success();
    let folded = crate_dir
        .join("target")
        .join("flameview")
        .join("debug")
        .join("eg.folded");
    assert!(folded.is_file());
}
