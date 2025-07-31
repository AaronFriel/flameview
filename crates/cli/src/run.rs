use std::fs;

use flameview::loader::{self, collapsed};

use crate::args::{Args, Command};

pub fn run(args: Args) -> Result<(), ()> {
    match args.cmd {
        Command::Summarize {
            file,
            max_lines,
            coverage,
        } => {
            let data = fs::read(&file).map_err(|_| {
                eprintln!("flameview: unable to read {}", file.display());
            })?;
            let tree = collapsed::load(data.as_slice()).map_err(|e| match e {
                loader::Error::Io(_) => {
                    eprintln!("flameview: unable to read {}", file.display());
                }
                loader::Error::BadLine(line) => {
                    eprintln!("flameview: parse error on line {line}");
                }
            })?;
            let summary = tree.summarize(max_lines, coverage);
            println!("{}", summary);
        }
    }
    Ok(())
}
