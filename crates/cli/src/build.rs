use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use anyhow::{anyhow, Context};
use cargo_metadata::{Message, MetadataCommand};

pub use cargo_metadata::{Artifact, ArtifactDebuginfo};

use crate::opts::{Opt, TargetKind};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BinaryTarget {
    pub package: String,
    pub target: String,
    pub kind: Vec<TargetKind>,
}

#[allow(dead_code)]
pub trait CommandExecutor {
    fn run(&self, cmd: &mut Command) -> io::Result<Output>;
}

pub struct RealCommandExecutor;

impl CommandExecutor for RealCommandExecutor {
    fn run(&self, cmd: &mut Command) -> io::Result<Output> {
        cmd.output()
    }
}

pub fn find_crate_root(manifest_path: Option<&Path>) -> anyhow::Result<PathBuf> {
    if let Some(mp) = manifest_path {
        let dir = mp
            .parent()
            .ok_or_else(|| anyhow!("invalid manifest path"))?;
        return std::fs::canonicalize(dir).context("canonicalize manifest dir");
    }
    let mut dir = std::env::current_dir().context("current dir")?;
    loop {
        if dir.join("Cargo.toml").is_file() {
            return Ok(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    Err(anyhow!("could not find Cargo.toml"))
}

pub fn find_unique_target(
    kind: &[TargetKind],
    pkg: Option<&str>,
    manifest_path: Option<&Path>,
    target_name: Option<&str>,
) -> anyhow::Result<BinaryTarget> {
    let mut cmd = MetadataCommand::new();
    cmd.no_deps();
    if let Some(mp) = manifest_path {
        cmd.manifest_path(mp);
    }
    let metadata = cmd.exec().context("metadata")?;

    let crate_root = find_crate_root(manifest_path)?;
    let packages = metadata.packages.into_iter().filter(|p| {
        if let Some(name) = pkg {
            p.name == name
        } else {
            p.manifest_path
                .as_std_path()
                .parent()
                .map(|d| d == crate_root)
                .unwrap_or(false)
        }
    });

    let mut candidates = Vec::new();
    let mut defaults = Vec::new();
    for p in packages {
        for t in &p.targets {
            let tkinds: Vec<TargetKind> = t.kind.iter().filter_map(|k| k.parse().ok()).collect();
            if tkinds.iter().any(|k| kind.contains(k)) {
                let bt = BinaryTarget {
                    package: p.name.clone(),
                    target: t.name.clone(),
                    kind: tkinds.clone(),
                };
                if let Some(def) = &p.default_run {
                    if def == &t.name {
                        defaults.push(bt.clone());
                    }
                }
                candidates.push(bt);
            }
        }
    }

    let mut list = if let Some(name) = target_name {
        candidates
            .into_iter()
            .filter(|t| t.target == name)
            .collect::<Vec<_>>()
    } else if defaults.len() == 1 {
        defaults
    } else {
        candidates
    };

    if list.len() == 1 {
        Ok(list.remove(0))
    } else if list.is_empty() {
        Err(anyhow!("no targets found for kinds {:?}", kind))
    } else {
        let names: Vec<_> = list.iter().map(|t| t.target.clone()).collect();
        Err(anyhow!("multiple targets match: {}", names.join(", ")))
    }
}

#[allow(dead_code)]
pub fn build(
    exec: &dyn CommandExecutor,
    opt: &Opt,
    kind: Vec<TargetKind>,
) -> anyhow::Result<Vec<Artifact>> {
    let mut cmd = Command::new("cargo");
    if !opt.dev && (opt.bench.is_some() || opt.unit_bench.is_some()) {
        cmd.args(["bench", "--no-run"]);
    } else if opt.unit_test.is_some() {
        cmd.args(["test", "--no-run"]);
    } else {
        cmd.arg("build");
    }
    if let Some(profile) = &opt.cargo_profile {
        cmd.args(["--profile", profile]);
    } else if opt.release {
        cmd.arg("--release");
    }
    if let Some(pkg) = &opt.package {
        cmd.args(["--package", pkg]);
    }
    let mut specified = false;
    if let Some(bin) = &opt.bin {
        cmd.args(["--bin", bin]);
        specified = true;
    }
    if let Some(example) = &opt.example {
        cmd.args(["--example", example]);
        specified = true;
    }
    if let Some(test) = &opt.test {
        cmd.args(["--test", test]);
        specified = true;
    }
    if let Some(bench) = &opt.bench {
        cmd.args(["--bench", bench]);
        specified = true;
    }
    if let Some(unit_test) = &opt.unit_test {
        specified = true;
        if let Some(name) = unit_test {
            cmd.args(["--test", name]);
        } else {
            cmd.arg("--tests");
        }
    }
    if let Some(unit_bench) = &opt.unit_bench {
        specified = true;
        if let Some(name) = unit_bench {
            cmd.args(["--bench", name]);
        } else {
            cmd.arg("--benches");
        }
    }
    if !specified && !kind.is_empty() {
        let bt = find_unique_target(
            &kind,
            opt.package.as_deref(),
            opt.manifest_path.as_deref(),
            None,
        )?;
        cmd.args(["--package", &bt.package]);
        if bt.kind.contains(&TargetKind::Bin) {
            cmd.args(["--bin", &bt.target]);
        } else if bt.kind.contains(&TargetKind::Example) {
            cmd.args(["--example", &bt.target]);
        } else if bt.kind.contains(&TargetKind::Test) {
            cmd.args(["--test", &bt.target]);
        } else if bt.kind.contains(&TargetKind::Bench) {
            cmd.args(["--bench", &bt.target]);
        } else {
            return Err(anyhow!("unknown target kind {:?}", bt.kind));
        }
    }
    if let Some(manifest) = &opt.manifest_path {
        cmd.args(["--manifest-path", manifest.to_str().unwrap()]);
    }
    if let Some(target) = &opt.target {
        cmd.args(["--target", target]);
    }
    if let Some(features) = &opt.features {
        cmd.args(["--features", features]);
    }
    if opt.no_default_features {
        cmd.arg("--no-default-features");
    }
    cmd.arg("--message-format=json-render-diagnostics");
    cmd.stderr(Stdio::inherit());
    let Output { status, stdout, .. } = exec.run(&mut cmd).map_err(|e| {
        if e.kind() == io::ErrorKind::NotFound {
            anyhow!("failed to execute `cargo`: not found in PATH")
        } else {
            anyhow!("failed to execute `cargo`: {e}")
        }
    })?;
    if !status.success() {
        return Err(anyhow!("cargo build failed"));
    }
    let mut artifacts = Vec::new();
    for message in Message::parse_stream(stdout.as_slice()).flatten() {
        if let Message::CompilerArtifact(artifact) = message {
            artifacts.push(artifact);
        }
    }
    Ok(artifacts)
}

pub type Workload = Vec<std::ffi::OsString>;

pub fn workload(opt: &Opt, artifacts: &[Artifact]) -> anyhow::Result<Workload> {
    if !artifacts.iter().any(|a| a.executable.is_some()) {
        return Err(anyhow!("no executable artifacts"));
    }
    let mut cmd: Workload = Vec::new();
    let (name, kind) = if let Some(bin) = &opt.bin {
        (bin.clone(), TargetKind::Bin)
    } else if let Some(example) = &opt.example {
        (example.clone(), TargetKind::Example)
    } else if let Some(test) = &opt.test {
        (test.clone(), TargetKind::Test)
    } else if let Some(bench) = &opt.bench {
        (bench.clone(), TargetKind::Bench)
    } else {
        let bt = find_unique_target(
            &[
                TargetKind::Bin,
                TargetKind::Example,
                TargetKind::Test,
                TargetKind::Bench,
            ],
            opt.package.as_deref(),
            opt.manifest_path.as_deref(),
            None,
        )?;
        let k = bt
            .kind
            .iter()
            .find(|k| {
                matches!(
                    k,
                    TargetKind::Bin | TargetKind::Example | TargetKind::Test | TargetKind::Bench
                )
            })
            .cloned()
            .ok_or_else(|| anyhow!("unknown target kind {:?}", bt.kind))?;
        (bt.target, k)
    };
    let artifact = artifacts
        .iter()
        .find(|a| a.target.name == name && a.target.kind.iter().any(|k| k == kind.as_str()))
        .ok_or_else(|| anyhow!("target artifact not found"))?;
    let exe = artifact
        .executable
        .as_ref()
        .ok_or_else(|| anyhow!("selected artifact has no executable"))?;
    if !opt.dev && artifact.profile.debuginfo == ArtifactDebuginfo::None {
        eprintln!("warning: selected profile lacks debuginfo");
    }
    cmd.push(exe.clone().into_std_path_buf().into_os_string());
    cmd.extend(opt.trailing_arguments.iter().cloned().map(Into::into));
    Ok(cmd)
}
