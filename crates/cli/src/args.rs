use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Summarize a collapsed stack file (use `-` for stdin)
    Summarize {
        /// Input file to read
        file: std::path::PathBuf,
        /// Maximum number of lines to display
        #[arg(long, default_value_t = 50)]
        max_lines: usize,
        /// Fraction of self samples to cover before stopping
        #[arg(long, default_value_t = 0.95)]
        coverage: f64,
    },
}
