use std::io::{self, BufRead, IsTerminal};
use std::time::Duration;

use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::Terminal;

use flameview::{input, loader::collapsed};

use crate::ViewArgs;

mod app;
use app::{Action, App};

pub fn tui(args: &ViewArgs) -> anyhow::Result<()> {
    let reader = input::open_source(&args.file)?;
    tui_from_reader(args, reader)
}

/// Internal helper so tests can inject their own reader.
pub fn tui_from_reader<R: BufRead>(args: &ViewArgs, reader: R) -> anyhow::Result<()> {
    let tree = collapsed::load_stream(reader).map_err(|e| crate::map_loader_err(e, &args.file))?;

    if !io::stdout().is_terminal() {
        println!("{}", tree.summarize(args.max_lines, args.coverage));
        return Ok(());
    }

    enable_raw_mode()?;
    let mut terminal = setup_terminal()?;
    let mut app = App::new(&tree);

    let res = run_app(&mut terminal, &mut app);
    restore_terminal(&mut terminal)?;
    res
}

#[allow(unused_imports)]
pub(crate) use tui_from_reader as tui_with_reader;

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
