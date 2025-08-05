use super::*;

use std::fs;
use std::io::Read;

use ratatui::{backend::Backend, buffer::Buffer, crossterm::event::KeyEvent, Terminal};

pub trait BufferBackend: Backend {
    fn get_buffer(&self) -> &Buffer;
}

impl BufferBackend for ratatui::backend::TestBackend {
    fn get_buffer(&self) -> &Buffer {
        self.buffer()
    }
}

pub fn run_with_backend<B>(
    args: &ViewArgs,
    backend: B,
    script: impl IntoIterator<Item = KeyEvent>,
) -> anyhow::Result<Vec<Buffer>>
where
    B: BufferBackend,
{
    let data = if args.file.as_os_str() == "-" {
        let mut buf = Vec::new();
        std::io::stdin().read_to_end(&mut buf)?;
        buf
    } else {
        fs::read(&args.file)?
    };

    let tree = collapsed::load_slice(data.as_slice()).map_err(|e| match e {
        loader::Error::Io(err) => err.into(),
        loader::Error::BadLine(line) => anyhow::anyhow!("parse error on line {line}"),
    })?;

    let mut term = Terminal::new(backend)?;
    let mut app = App::new(&tree, args.max_lines, args.coverage);
    let mut buffers = Vec::new();

    for ev in script {
        term.draw(|f| app.draw(f))?;
        buffers.push(term.backend().get_buffer().clone());
        if let Action::Exit = app.on_key(ev.code) {
            break;
        }
    }

    Ok(buffers)
}
