#[derive(Debug)]
pub enum Error {
    ParseFailure(String),
    MissingHeader(&'static str),
    IO(std::io::Error),
    Fmt(std::fmt::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<std::fmt::Error> for Error {
    fn from(source: std::fmt::Error) -> Self {
        Error::Fmt(source)
    }
}

impl From<&str> for Error {
    fn from(source: &str) -> Self {
        Error::ParseFailure(source.into())
    }
}
