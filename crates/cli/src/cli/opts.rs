use crate::profile::ProfileOptions;
pub use cargo_metadata::TargetKind;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Opt {
    #[arg(long)]
    pub dev: bool,
    #[arg(long)]
    #[arg(long = "profile")]
    pub cargo_profile: Option<String>,
    #[arg(long)]
    pub package: Option<String>,
    #[arg(long)]
    pub bin: Option<String>,
    #[arg(long)]
    pub example: Option<String>,
    #[arg(long)]
    pub test: Option<String>,
    #[arg(long)]
    pub bench: Option<String>,
    #[arg(long, value_name = "NAME", num_args = 0..=1)]
    pub unit_test: Option<Option<String>>,
    #[arg(long, value_name = "NAME", num_args = 0..=1)]
    pub unit_bench: Option<Option<String>>,
    #[arg(long, value_enum)]
    pub unit_test_kind: Option<UnitTestTargetKind>,
    #[arg(long)]
    pub manifest_path: Option<PathBuf>,
    #[arg(long)]
    pub target: Option<String>,
    #[arg(long)]
    pub features: Option<String>,
    #[arg(long)]
    pub no_default_features: bool,
    #[arg(long)]
    pub release: bool,
    #[clap(flatten)]
    pub profile: ProfileOptions,
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub trailing_arguments: Vec<String>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum UnitTestTargetKind {
    Bin,
    Lib,
}
