mod decoder;
mod encoder;

use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::str::FromStr;

pub use decoder::SSDPDecoder;
pub use encoder::SSDPEncoder;

use crate::errors::Error;

pub(crate) const REQUEST_LINE_NOTIFY: &str = "NOTIFY * HTTP/1.1";
pub(crate) const REQUEST_LINE_M_SEARCH: &str = "M-SEARCH * HTTP/1.1";
pub(crate) const REQUEST_LINE_OK: &str = "HTTP/1.1 200 OK";
pub(crate) const SSDP_ADDRESS: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
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
    typ: PacketType,
    headers: HashMap<String, String>,
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

#[derive(Debug, PartialEq)]
pub enum Message {
    MSearch {
        /// Field value contains maximum wait time in seconds. shall be greater than or equal to 1 and should
        /// be less than 5 inclusive. Device responses should be delayed a random duration between 0 and this many
        /// seconds to balance load for the control point when it processes responses. This value is allowed to be
        /// increased if a large number of devices are expected to respond.
        max_wait: u8,
        /// Field value contains Search Target.
        // TODO: enum
        target: String,
        /// Field value shall begin with the following “product tokens” (defined
        /// by HTTP/1.1). The first product token identifes the operating system in the form OS name/OS version, the
        /// second token represents the UPnP version and shall be UPnP/2.0, and the third token identifes the product
        /// using the form product name/product version. For example, “USER-AGENT: unix/5.1 UPnP/2.0
        /// MyProduct/1.0”.
        user_agent: Option<String>,
        /// control point can request that a device replies to a TCP port on the control point. When this header
        /// is present it identifies the TCP port on which the device can reply to the search.
        tcp_port: Option<u16>,
        /// Specifies the friendly name of the control point. The friendly name is vendor specific.
        friendly_name: String,
        /// uuid of the control point. When the control point is implemented in a UPnP device it is recommended
        /// to use the UDN of the co-located UPnP device. When implemented, all specified requirements for uuid usage
        /// in devices also apply for control points.
        uuid: Option<String>,
    },
    Available {
        notification_type: String, // TODO: Enum
        server: String,
        unique_service_name: String,
    },
    Unimplemented,
}
