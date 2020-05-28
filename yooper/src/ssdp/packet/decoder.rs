use crate::errors::Error;
use bytes::BytesMut;
use std::collections::HashMap;
use tokio_util::codec;

use super::Packet;

#[derive(Default)]
pub struct Decoder {}

impl codec::Decoder for Decoder {
    type Item = Packet;
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

        let typ = reqline.parse()?;

        let headers: HashMap<String, String> =
            iter.map(split_header).collect::<Result<_, Error>>()?;

        Ok(Some(Packet { typ, headers }))
    }
}

const MSG_END: [u8; 4] = [b'\r', b'\n', b'\r', b'\n'];

fn find_end(src: &BytesMut) -> Option<usize> {
    src.windows(4)
        .enumerate()
        .find(|(_, win)| win == &MSG_END)
        .map(|(i, _)| i + 2) // include the trailing \r\n
}

fn split_header(line: &str) -> Result<(String, String), Error> {
    let index = line
        .find(':')
        .ok_or_else(|| Error::ParseFailure(format!("unparseable header line: {}", line)))?;
    let (key, val) = line.split_at(index);
    let value = val[1..] // trim colon
        .trim_start() // trim space
        .into();
    Ok((key.to_lowercase(), value))
}

#[cfg(test)]
mod tests {
    use tokio_util::codec::Decoder;
    use bytes::BytesMut;
    use crate::ssdp::{packet::{Packet, PacketType}, tests::constants::*};

    #[test]
    fn test_parse_notify() {
        let mut buf = BytesMut::from(NOTIFY_EXAMPLE);

        let decoded = super::Decoder {}.decode(&mut buf).unwrap().unwrap();
        assert_eq!(
            decoded,
            Packet::new_from_literal(
                PacketType::Notify,
                vec![("host", "239.255.255.250:1900"),
                ("cache-control", "max-age=3600"),
                ("location", "http://192.168.7.238:54216/RootDevice.xml"),
                ("nt","urn:schemas-upnp-org:device:MediaServer:1"),
                ("nts", "ssdp:alive"),
                ("server","Windows 10/10.0 UPnP/1.0 Azureus/5.7.6.0"),
                ("usn","uuid:07853410-ccef-9e3c-de6a-410b371182eb::urn:schemas-upnp-org:device:MediaServer:1" )
                ]
            )
        );
    }

    #[test]
    fn test_parse_m_search() {
        let mut buf = BytesMut::from(M_SEARCH_EXAMPLE);
        let decoded = super::Decoder {}.decode(&mut buf).unwrap().unwrap();

        assert_eq!(
            decoded,
            Packet::new_from_literal(
                PacketType::MSearch,
                vec![
                    ("host", "239.255.255.250:1900"),
                    ("man", "\"ssdp:discover\""),
                    ("mx", "1"),
                    ("st", "urn:dial-multiscreen-org:service:dial:1"),
                    ("user-agent", "Chromium/81.0.4044.138 Linux"),
                ],
            )
        )
    }

    #[test]
    fn test_parse_search_response() {
        let mut buf = BytesMut::from(SEARCH_RESPONSE_EXAMPLE);
        let decoded = super::Decoder {}.decode(&mut buf).unwrap().unwrap();

        assert_eq!(
            decoded,
            Packet::new_from_literal(
                PacketType::Ok,
                vec![
                    ("cache-control", "max-age=1800"),
                    ("date", "Mon, 25 May 2020 02:39:02 GMT"),
                    ("ext", ""),
                    ("location", "http://192.168.7.1:1900/igd.xml"),
                    ("server", "eeroOS/latest UPnP/1.0 eero/latest"),
                    ("st", "uuid:fcdb9233-a63f-41da-b42c-7cfeb99c8adf"),
                    ("usn", "uuid:fcdb9233-a63f-41da-b42c-7cfeb99c8adf")
                ],
            )
        )
    }
}
