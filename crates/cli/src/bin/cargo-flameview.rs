use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Opt {
    #[arg(long)]
    dev: bool,
    #[arg(long)]
    profile: Option<String>,
    #[arg(long)]
    package: Option<String>,
    #[arg(long)]
    bin: Option<String>,
    #[arg(long)]
    example: Option<String>,
    #[arg(long)]
    test: Option<String>,
    #[arg(long)]
    bench: Option<String>,
    #[arg(long, value_name = "NAME", num_args = 0..=1)]
    unit_test: Option<Option<String>>,
    #[arg(long, value_name = "NAME", num_args = 0..=1)]
    unit_bench: Option<Option<String>>,
    #[arg(long, value_enum)]
    unit_test_kind: Option<UnitTestTargetKind>,
    #[arg(long)]
    manifest_path: Option<PathBuf>,
    #[arg(long)]
    target: Option<String>,
    #[arg(long)]
    features: Option<String>,
    #[arg(long)]
    no_default_features: bool,
    #[arg(long)]
    release: bool,
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    trailing_arguments: Vec<String>,
}

#[derive(ValueEnum, Clone, Debug)]
enum UnitTestTargetKind {
    Bin,
    Lib,
}

fn main() {
    let opt = Opt::parse();
    println!("{opt:#?}");
}
