pub mod run;
pub mod summarize;
pub mod viewer;

use clap::Parser;
use flameview::loader;
use std::path::{Path, PathBuf};

#[derive(Parser, Clone)]
pub struct ViewArgs {
    /// Folded stack file to open ("-" for stdin)
    pub file: PathBuf,
    /// Print top-N summary instead of interactive view
    #[arg(long)]
    pub summarize: bool,
    /// Maximum summary lines (only with --summarize)
    #[arg(long, default_value_t = 50)]
    pub max_lines: usize,
    /// Coverage fraction for summary
    #[arg(long, default_value_t = 0.95)]
    pub coverage: f64,
}

pub(crate) fn map_loader_err(e: loader::Error, file: &Path) -> anyhow::Error {
    match e {
        loader::Error::Io(_) => {
            if file.as_os_str() == "-" {
                anyhow::anyhow!("flameview: unable to read stdin")
            } else {
                anyhow::anyhow!("flameview: unable to read {}", file.display())
            }
        }
        loader::Error::BadLine(line) => anyhow::anyhow!("flameview: parse error on line {line}"),
    }
}
