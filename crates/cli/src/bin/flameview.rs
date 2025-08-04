use clap::Parser;
use flameview_cli::{run, viewer, ViewArgs};

fn main() -> anyhow::Result<()> {
    let args = ViewArgs::parse();
    if args.summarize {
        run::summarize(args)?;
    } else {
        viewer::tui(&args)?;
    }
    Ok(())
}
