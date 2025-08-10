use std::cell::Cell;
use std::io::{self, BufRead, Cursor, Read};
use std::path::PathBuf;
use std::rc::Rc;

use flameview::loader::{self, collapsed};
use flameview_cli::{viewer, ViewArgs};

#[test]
fn bad_line_number_zero_based() {
    let data = b"a;b 1\nc;d 2\nfoo\n";
    let err = collapsed::load_stream(Cursor::new(&data[..]))
        .err()
        .unwrap();
    assert!(matches!(err, loader::Error::BadLine(3)));
}

struct CountingReader<R> {
    inner: R,
    eof_reads: Rc<Cell<usize>>,
}

impl<R: Read> Read for CountingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: BufRead> BufRead for CountingReader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        let b = self.inner.fill_buf()?;
        if b.is_empty() {
            let c = self.eof_reads.get();
            self.eof_reads.set(c + 1);
        }
        Ok(b)
    }
    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}

#[test]
fn stdin_once() {
    let data = b"a;b 1\n";
    let eof = Rc::new(Cell::new(0));
    let reader = CountingReader {
        inner: Cursor::new(&data[..]),
        eof_reads: eof.clone(),
    };
    let args = ViewArgs {
        file: PathBuf::from("-"),
        summarize: false,
        max_lines: 10,
        coverage: 0.9,
    };
    viewer::tui_from_reader(&args, reader).unwrap();
    assert_eq!(eof.get(), 1);
}

#[cfg(feature = "large-tests")]
#[test]
fn large_stream_memory() {
    use flameview::loader::collapsed;

    fn mem_usage() -> usize {
        let status = std::fs::read_to_string("/proc/self/status").unwrap();
        for line in status.lines() {
            if let Some(kb) = line.strip_prefix("VmHWM:") {
                let kb = kb.trim().split_whitespace().next().unwrap();
                return kb.parse::<usize>().unwrap() * 1024;
            }
        }
        0
    }

    let mut data = Vec::new();
    let line = b"f 1\n";
    while data.len() < 200 * 1024 * 1024 {
        data.extend_from_slice(line);
    }
    let before = mem_usage();
    let _tree = collapsed::load_stream(Cursor::new(&data)).unwrap();
    let after = mem_usage();
    assert!(after - before < 400 * 1024 * 1024);
}
