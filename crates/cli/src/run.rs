use anyhow::{Context, Result};
use flameview::input;

use crate::{map_loader_err, summarize, ViewArgs};

pub fn summarize(args: ViewArgs) -> Result<()> {
    let file = args.file;
    let reader = input::open_source(&file).with_context(|| {
        if file.as_os_str() == "-" {
            "flameview: unable to read stdin".to_string()
        } else {
            format!("flameview: unable to read {}", file.display())
        }
    })?;
    let summary = summarize::summarize(reader, args.max_lines, args.coverage)
        .map_err(|e| map_loader_err(e, &file))?;
    println!("{summary}");
    Ok(())
}
