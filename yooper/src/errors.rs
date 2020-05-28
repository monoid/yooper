use thiserror::Error;

use std::convert::Infallible;
use std::num::ParseIntError;
use mac_address::MacAddressError;
use uuid;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Parse failure: {0}")]
    ParseFailure(String),

    #[error("couldn't parse integer")]
    ParseIntError(#[from] ParseIntError),

    #[error("missing required header {0}")]
    MissingHeader(&'static str),

    #[error("Required constant header had incorrect value {0}")]
    IncorrectHeader(&'static str),

    #[error("IO Error {0}")]
    IO(#[from] std::io::Error),

    #[error("Format Error")]
    Fmt(#[from]std::fmt::Error),

    #[error("Received a packet we don't understand")]
    UnknownPacket, // TODO EKF more descriptive

    #[error("Couldn't discover this computer's MAC address")]
    MACAddressError(#[from] MacAddressError),

    #[error("if you see this, something's wrong")]
    Infallible(#[from] Infallible),

    #[error("Failed to generate a UUID")]
    UUID(#[from] uuid::Error),
}
