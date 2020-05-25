mod decoder;
mod encoder;

use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::str::FromStr;

use crate::errors::Error;

pub use yooper_derive::*;

pub(crate) const REQUEST_LINE_NOTIFY: &str = "NOTIFY * HTTP/1.1";
pub(crate) const REQUEST_LINE_M_SEARCH: &str = "M-SEARCH * HTTP/1.1";
pub(crate) const REQUEST_LINE_OK: &str = "HTTP/1.1 200 OK";
#[allow(dead_code)]
pub(crate) const SSDP_ADDRESS: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
#[allow(dead_code)]
pub(crate) const SSDP_PORT: u16 = 1900;

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

#[derive(PartialEq, Debug)]
pub struct Packet {
    pub typ: PacketType,
    pub headers: HashMap<String, String>,
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

pub trait FromPacket: std::marker::Sized {
    fn from_packet(msg: &Packet) -> Result<Self, crate::errors::Error>;
}

pub trait ToPacket {
    fn to_packet(&self) -> Packet;
}
