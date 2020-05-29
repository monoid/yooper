use thiserror::Error;

use mac_address::MacAddressError;
use std::convert::Infallible;
use std::num::ParseIntError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Parse failure: {0}")]
    ParseFailure(String),

    #[error("couldn't parse integer")]
    ParseIntError(#[from] ParseIntError),

    #[error("missing required header {0}")]
    MissingHeader(&'static str),

    #[error("Required header {0} had incorrect value")]
    IncorrectHeader(&'static str),

    #[error("Header {0} had a value we couldn't parse ({1})")]
    MalformedHeader(&'static str, String),

    #[error("IO Error {0}")]
    IO(#[from] std::io::Error),

    #[error("Format Error")]
    Fmt(#[from] std::fmt::Error),

    #[error("Received a packet we don't understand")]
    UnknownPacket, // TODO EKF more descriptive

    #[error("Couldn't discover this computer's MAC address")]
    MACAddressError(#[from] MacAddressError),

    #[error("if you see this, something's wrong")]
    Infallible(#[from] Infallible),

    #[error("Failed to generate a UUID")]
    UUID(#[from] uuid::Error),
}
