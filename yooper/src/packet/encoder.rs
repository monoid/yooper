use bytes::BytesMut;
use tokio_util::codec::Encoder;

use super::Packet;
use crate::errors::Error;
use std::fmt::Write;

#[derive(Default)]
pub struct SSDPEncoder {}

impl Encoder<Packet> for SSDPEncoder {
    type Error = Error;

    #[allow(clippy::write_with_newline)]
    fn encode(&mut self, p: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        write!(dst, "{}\r\n", p.typ.to_string())?;
        p.headers
            .iter()
            .map(|(k, v)| write!(dst, "{}: {}\r\n", k, v))
            .collect::<Result<(), std::fmt::Error>>()?;
        write!(dst, "\r\n")?;
        Ok(())
    }
}

// impl Encoder<Message> for SSDPEncoder {
//     type Error = Error;
//     #[allow(clippy::write_with_newline)]
//     fn encode(&mut self, m: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
//         use Message::*;

//         match m {
//             MSearch {
//                 max_wait,
//                 target,
//                 user_agent,
//                 tcp_port,
//                 friendly_name,
//                 uuid,
//             } => {
//                 write!(dst, "M-SEARCH * HTTP/1.1\r\n")?;
//                 write_header(dst, "host", &format!("{}:{}", SSDP_ADDRESS, SSDP_PORT))?;
//                 write_header(dst, "man", "\"ssdp:discover\"")?;
//                 write_header(dst, "mx", &max_wait.to_string())?;
//                 write_header(dst, "st", &target)?;
//                 maybe_write(dst, "user-agent", user_agent)?;
//                 maybe_write(dst, "tcpport.upnp.org", tcp_port)?;
//                 write_header(dst, "cpfn.upnp.org", &friendly_name)?;
//                 maybe_write(dst, "cpuuid.upnp.org", uuid)?;
//                 write!(dst, "\r\n")?;
//             }
//             _ => unimplemented!(),
//         }

//         Ok(())
//     }
// }
