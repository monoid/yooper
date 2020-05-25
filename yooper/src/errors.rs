use std::convert::Infallible;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    ParseFailure(String),
    ParseIntError(ParseIntError),
    MissingHeader(&'static str),
    IncorrectHeader(&'static str),
    IO(std::io::Error),
    Fmt(std::fmt::Error),
    UnknownPacket, // TODO EKF more descriptive
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

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ParseIntError(e)
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
