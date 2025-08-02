#![cfg(not(miri))]

use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn parse_help_contains_flags() {
    let assert = Command::new("cargo")
        .args(["run", "--bin", "cargo-flameview", "--", "--help"])
        .assert()
        .success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let stdout = stdout
        .lines()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join("\n");
    insta::assert_snapshot!(stdout, @r###"Usage: cargo-flameview [OPTIONS] [TRAILING_ARGUMENTS]...

Arguments:
  [TRAILING_ARGUMENTS]...

Options:
      --dev
      --profile <PROFILE>
      --package <PACKAGE>
      --bin <BIN>
      --example <EXAMPLE>
      --test <TEST>
      --bench <BENCH>
      --unit-test [<NAME>]
      --unit-bench [<NAME>]
      --unit-test-kind <UNIT_TEST_KIND>  [possible values: bin, lib]
      --manifest-path <MANIFEST_PATH>
      --target <TARGET>
      --features <FEATURES>
      --no-default-features
      --release
  -h, --help                             Print help
  -V, --version                          Print version
"###);
}

#[test]
fn parse_trailing_arguments() {
    let assert = Command::new("cargo")
        .args(["run", "--bin", "cargo-flameview", "--", "--", "foo", "bar"])
        .assert()
        .success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let trailing = stdout
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join(" ");
    insta::assert_snapshot!(trailing, @"foo bar");
}
