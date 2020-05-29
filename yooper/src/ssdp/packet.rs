//! Packet is an unstructured intermediate representation of an SSDP UDP packet
mod decoder;
mod encoder;

use indexmap::IndexMap;
use std::str::FromStr;

use crate::Error;

pub use decoder::Decoder;
pub use encoder::Encoder;
pub use yooper_derive::{FromHeaders, FromPacket, ToHeaders, ToPacket};

const REQUEST_LINE_NOTIFY: &str = "NOTIFY * HTTP/1.1";
const REQUEST_LINE_M_SEARCH: &str = "M-SEARCH * HTTP/1.1";
const REQUEST_LINE_OK: &str = "HTTP/1.1 200 OK";

/// The Request line of the packet
#[derive(PartialEq, Debug)]
pub enum PacketType {
    MSearch,
    Notify,
    Ok,
}

impl ToString for PacketType {
    fn to_string(&self) -> String {
        match self {
            Self::MSearch => REQUEST_LINE_M_SEARCH,
            Self::Notify => REQUEST_LINE_NOTIFY,
            Self::Ok => REQUEST_LINE_OK,
        }
        .to_string()
    }
}

impl FromStr for PacketType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            REQUEST_LINE_OK => Ok(Self::Ok),
            REQUEST_LINE_NOTIFY => Ok(Self::Notify),
            REQUEST_LINE_M_SEARCH => Ok(Self::MSearch),
            s => Err(Error::ParseFailure(format!("Unknown request line {}", s))),
        }
    }
}

/// records, in order, the headers for the packet
pub type Headers = IndexMap<String, String>;

/// A single SSDP packet
#[derive(PartialEq, Debug)]
pub struct Packet {
    /// The request line of a packet
    pub typ: PacketType,
    /// The headers from the packet
    pub headers: Headers,
}

impl Packet {
    #[cfg(test)]
    pub(crate) fn new_from_literal(typ: PacketType, headers: Vec<(&str, &str)>) -> Self {
        let headers = headers
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        Self { typ, headers }
    }
}

/// Deserialize a packet into something more structured
pub trait FromPacket: std::marker::Sized {
    fn from_packet(msg: &Packet) -> Result<Self, crate::errors::Error>;
}

/// Serialize a structured representation into a packet
pub trait ToPacket {
    fn to_packet(&self) -> Packet;
}

pub trait FromHeaders: std::marker::Sized {
    fn from_headers(headers: &Headers) -> Result<Self, crate::errors::Error>;
}

pub trait ToHeaders {
    fn to_headers(&self) -> Headers;
}
