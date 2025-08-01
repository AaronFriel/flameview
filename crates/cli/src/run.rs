use std::fs;
use std::io::{self, Read};

use flameview::loader::{self, collapsed};

use crate::args::{Args, Command};

pub fn run(args: Args) -> Result<(), ()> {
    match args.cmd {
        Command::Summarize {
            file,
            max_lines,
            coverage,
        } => {
            let data = if file.as_os_str() == "-" {
                let mut buf = Vec::new();
                io::stdin().read_to_end(&mut buf).map_err(|_| {
                    eprintln!("flameview: unable to read stdin");
                })?;
                buf
            } else {
                fs::read(&file).map_err(|_| {
                    eprintln!("flameview: unable to read {}", file.display());
                })?
            };
            let tree = collapsed::load_slice(data.as_slice()).map_err(|e| match e {
                loader::Error::Io(_) => {
                    if file.as_os_str() == "-" {
                        eprintln!("flameview: unable to read stdin");
                    } else {
                        eprintln!("flameview: unable to read {}", file.display());
                    }
                }
                loader::Error::BadLine(line) => {
                    eprintln!("flameview: parse error on line {line}");
                }
            })?;
            let summary = tree.summarize(max_lines, coverage);
            println!("{summary}");
        }
    }
    Ok(())
}
