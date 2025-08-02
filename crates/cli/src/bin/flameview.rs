#[path = "../args.rs"]
mod args;
#[path = "../run.rs"]
mod run;

use clap::Parser;

fn main() {
    let args = args::Args::parse();
    if run::run(args).is_err() {
        std::process::exit(1);
    }
}
