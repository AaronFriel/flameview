use std::fs;
use std::io::{self, Read};

use anyhow::{anyhow, Context, Result};
use flameview::loader::{self, collapsed};

use crate::ViewArgs;

pub fn summarize(args: ViewArgs) -> Result<()> {
    let file = args.file;
    let data = if file.as_os_str() == "-" {
        let mut buf = Vec::new();
        io::stdin()
            .read_to_end(&mut buf)
            .context("flameview: unable to read stdin")?;
        buf
    } else {
        fs::read(&file).with_context(|| format!("flameview: unable to read {}", file.display()))?
    };
    let tree = collapsed::load_slice(data.as_slice()).map_err(|e| match e {
        loader::Error::Io(_) => {
            if file.as_os_str() == "-" {
                anyhow!("flameview: unable to read stdin")
            } else {
                anyhow!("flameview: unable to read {}", file.display())
            }
        }
        loader::Error::BadLine(line) => anyhow!("flameview: parse error on line {line}"),
    })?;
    let summary = tree.summarize(args.max_lines, args.coverage);
    println!("{summary}");
    Ok(())
}
