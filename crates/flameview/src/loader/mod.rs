pub mod collapsed;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    BadLine(usize),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
