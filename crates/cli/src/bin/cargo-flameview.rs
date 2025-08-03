#[path = "../build.rs"]
mod build;
#[path = "../cli/opts.rs"]
mod opts;
#[path = "../profile.rs"]
mod profile;

use clap::Parser;
use opts::{Opt, TargetKind};

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();
    let kinds = if opt.example.is_some() {
        vec![TargetKind::Example]
    } else if opt.test.is_some() || opt.unit_test.is_some() {
        vec![TargetKind::Test]
    } else if opt.bench.is_some() || opt.unit_bench.is_some() {
        vec![TargetKind::Bench]
    } else {
        vec![TargetKind::Bin, TargetKind::Example]
    };
    let exec = build::RealCommandExecutor;
    let artifacts = build::build(&exec, &opt, kinds)?;
    let cmd = build::workload(&opt, &artifacts)?;
    let workload = cmd
        .iter()
        .map(|s| s.to_string_lossy().to_string())
        .collect::<Vec<_>>();
    let display = workload.join(" ");
    println!("{display}");
    let result = profile::profile(&exec, &workload, &opt)?;
    println!("folded: {}", result.folded_path.display());
    Ok(())
}
