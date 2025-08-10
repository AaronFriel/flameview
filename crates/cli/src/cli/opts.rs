use crate::profile::ProfileOptions;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetKind {
    Bin,
    Example,
    Test,
    Bench,
}

impl TargetKind {
    pub fn as_str(self) -> &'static str {
        match self {
            TargetKind::Bin => "bin",
            TargetKind::Example => "example",
            TargetKind::Test => "test",
            TargetKind::Bench => "bench",
        }
    }
}

impl std::str::FromStr for TargetKind {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bin" => Ok(TargetKind::Bin),
            "example" => Ok(TargetKind::Example),
            "test" => Ok(TargetKind::Test),
            "bench" => Ok(TargetKind::Bench),
            _ => Err(()),
        }
    }
}

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
