use std::fs;
use std::io::{self, IsTerminal, Read};
use std::time::Duration;

use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::Terminal;

use flameview::loader::{self, collapsed};

use crate::{run, ViewArgs};

mod app;
use app::{Action, App};

#[cfg(debug_assertions)]
mod test_harness;
#[cfg(debug_assertions)]
pub use test_harness::run_with_backend;

pub fn tui(args: &ViewArgs) -> anyhow::Result<()> {
    let data = if args.file.as_os_str() == "-" {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        buf
    } else {
        fs::read(&args.file)?
    };

    let tree = collapsed::load_slice(data.as_slice()).map_err(|e| match e {
        loader::Error::Io(err) => err.into(),
        loader::Error::BadLine(line) => anyhow::anyhow!("parse error on line {line}"),
    })?;

    if !io::stdout().is_terminal() {
        run::summarize(args.clone())?;
        return Ok(());
    }

    enable_raw_mode()?;
    let mut terminal = setup_terminal()?;
    let mut app = App::new(&tree);

    let res = run_app(&mut terminal, &mut app);
    restore_terminal(&mut terminal)?;
    res
}

type TuiTerminal = Terminal<CrosstermBackend<std::io::Stdout>>;

fn setup_terminal() -> anyhow::Result<TuiTerminal> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).map_err(Into::into)
}

fn restore_terminal(terminal: &mut TuiTerminal) -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn run_app(terminal: &mut TuiTerminal, app: &mut App) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| app.draw(f))?;
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if let Action::Exit = app.on_key(key.code) {
                    break;
                }
            }
        }
    }
    Ok(())
}
