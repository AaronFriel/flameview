pub mod run;
pub mod viewer;

use clap::Parser;
use std::path::PathBuf;

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
