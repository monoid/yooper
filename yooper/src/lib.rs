//! Yooper is a library for discovering and controlling UPnP devices on your network.
//! The modules represent the phases of UPnP: First Discovery, then Description.
#[cfg(feature = "description")]
pub mod description;
pub mod discovery;
mod errors;
pub mod ssdp;

pub use errors::Error;
