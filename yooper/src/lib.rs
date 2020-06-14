#[cfg(feature = "description")]
pub mod description;
pub mod discovery;
mod errors;
pub mod ssdp;

pub use errors::Error;
