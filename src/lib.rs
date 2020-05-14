mod tests;

use bytes::BytesMut;
use std::collections::HashMap;
use std::fmt::Write;
use std::net::Ipv4Addr;
use tokio_util::codec::{Decoder, Encoder};

const SSDP_ADDRESS: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
const SSDP_PORT: u16 = 1900;
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, PartialEq)]
pub enum Message {
    MSearch {
        target: String,
        mx: i32,
        uuid: Option<String>,
        friendly_name: Option<String>,
    },
    Available {
        notification_type: String, // TODO: Enum
        server: String,
        unique_service_name: String,
    },
    Unimplemented,
}

#[derive(Debug)]
enum Error {
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

struct SSDPEncoder {}

impl Encoder<Message> for SSDPEncoder {
    type Error = Error;
    #[allow(clippy::write_with_newline)]
    fn encode(&mut self, m: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        use Message::*;

        match m {
            MSearch {
                target, mx, uuid, ..
            } => {
                write!(dst, "M-SEARCH * HTTP/1.1\r\n")?;
                write_header(dst, "host", &format!("{}:{}", SSDP_ADDRESS, SSDP_PORT))?;
                write_header(dst, "man", "\"ssdp:discover\"")?;
                write_header(dst, "mx", &mx.to_string())?;
                write_header(dst, "st", &target)?;
                write_header(
                    dst,
                    "user-agent",
                    &format!("linux/5.1 UpnP/2.0 yooper/{}", VERSION),
                )?;
                write_header(dst, "cpfn.upnp.org", "yooper")?;
                if let Some(uuid) = uuid {
                    write_header(dst, "cpuuid.upnp.org", &uuid)?;
                }
                write!(dst, "\r\n")?;
            }
            _ => unimplemented!(),
        }

        Ok(())
    }
}

#[allow(clippy::write_with_newline)]
fn write_header(dst: &mut BytesMut, key: &str, value: &str) -> Result<(), std::fmt::Error>
where
{
    write!(dst, "{}: {}\r\n", key, value)
}

struct SSDPDecoder {}

const REQUEST_LINE_NOTIFY: &str = "NOTIFY * HTTP/1.1";
const REQUEST_LINE_M_SEARCH: &str = "M-SEARCH * HTTP/1.1";
const REQUEST_LINE_OK: &str = "HTTP/1.1 200 OK";

const SSDP_ALIVE: &str = "ssdp:alive";

impl Decoder for SSDPDecoder {
    type Item = Message;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let end = match find_end(src) {
            None => return Ok(None),
            Some(i) => i,
        };

        let buf = src.split_to(end);
        let bufstr = String::from_utf8_lossy(&buf[..buf.len() - 2]); // leave off last `\r\n`
        let mut iter = bufstr.split("\r\n");
        let reqline = iter
            .next()
            .ok_or_else(|| Error::ParseFailure("missing request line".into()))?;
        let headers: HashMap<String, String> =
            iter.map(split_header).collect::<Result<_, Error>>()?;

        if reqline == REQUEST_LINE_M_SEARCH {
            // TODO: check Host
            // TODO: man == "ssdp:discover"
            let target = headers
                .get("st")
                .ok_or("missing required header st")?
                .into();
            let mx: i32 = headers
                .get("mx")
                .ok_or("missing required header mx")?
                .parse()
                .map_err(|_| "invalid mx")?;
            let uuid = headers.get("cpuuid.upnp.org").map(|u| u.into());
            let friendly_name = headers.get("cpfn.upnp.org").map(|f| f.into());
            return Ok(Some(Message::MSearch {
                target,
                mx,
                uuid,
                friendly_name,
            }));
        }

        if dbg!(reqline) == REQUEST_LINE_NOTIFY {
            match dbg!(get_header(&headers, "nts")?.as_ref()) {
                SSDP_ALIVE => {
                    let notification_type = get_header(&headers, "nt")?;

                    let server = get_header(&headers, "server")?;
                    let unique_service_name = get_header(&headers, "usn")?;
                    return Ok(Some(Message::Available {
                        notification_type,
                        server,
                        unique_service_name,
                    }));
                }
                _ => (),
            }
        }
        Ok(Some(Message::Unimplemented))
    }
}

const MSG_END: [u8; 4] = [b'\r', b'\n', b'\r', b'\n'];

fn find_end(src: &BytesMut) -> Option<usize> {
    src.windows(4)
        .enumerate()
        .find(|(_, win)| win == &MSG_END)
        .map(|(i, _)| i + 1)
}

fn split_header(line: &str) -> Result<(String, String), Error> {
    let index = line
        .find(':')
        .ok_or_else(|| Error::ParseFailure(format!("unparseable header line: {}", line)))?;
    let (key, val) = line.split_at(index);
    Ok((key.to_lowercase(), val[1..].trim_start().into()))
}

fn get_header(map: &HashMap<String, String>, key: &'static str) -> Result<String, Error> {
    Ok(map.get(key).ok_or(Error::MissingHeader(key))?.into())
}
