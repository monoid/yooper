use crate::errors::Error;
use bytes::BytesMut;
use std::collections::HashMap;
use tokio_util::codec::Decoder;

use super::{Message, REQUEST_LINE_M_SEARCH, REQUEST_LINE_NOTIFY, SSDP_ALIVE};

pub struct SSDPDecoder {}

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
            //     // TODO: check Host
            //     // TODO: man == "ssdp:discover"
            //     let target = headers
            //         .get("st")
            //         .ok_or("missing required header st")?
            //         .into();
            //     let mx: i32 = headers
            //         .get("mx")
            //         .ok_or("missing required header mx")?
            //         .parse()
            //         .map_err(|_| "invalid mx")?;
            //     let uuid = headers.get("cpuuid.upnp.org").map(|u| u.into());
            //     let friendly_name = headers.get("cpfn.upnp.org").map(|f| f.into());
            //     return Ok(Some(Message::MSearch {
            //         target,
            //         mx,
            //         uuid,
            //         friendly_name,
            //     }));
        }

        if dbg!(reqline) == REQUEST_LINE_NOTIFY {
            match &(get_header(&headers, "nts")?) {
                "ssdp:alive" => {
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

#[cfg(test)]
mod tests {
    use super::*;
    const NOTIFY_EXAMPLE: &[u8] = include_bytes!("testdata/notify.bin");

    #[test]
    fn test_parse_notify() {
        let mut buf = BytesMut::from(NOTIFY_EXAMPLE);

        let decoded = SSDPDecoder {}.decode(&mut buf).unwrap().unwrap();
        assert_eq!(
            decoded,
            Message::Available {
                notification_type: "urn:schemas-upnp-org:device:MediaServer:1".to_string(),
                server: "Windows 10/10.0 UPnP/1.0 Azureus/5.7.6.0".to_string(),
                unique_service_name: "uuid:07853410-ccef-9e3c-de6a-410b371182eb::urn:schemas-upnp-org:device:MediaServer:1".to_string(),
            }
        )
    }
}
