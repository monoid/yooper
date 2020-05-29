use bytes::BytesMut;
use tokio_util::codec;

use super::Packet;
use crate::errors::Error;
use std::fmt::Write;

#[derive(Default)]
pub struct Encoder {}

impl codec::Encoder<Packet> for Encoder {
    type Error = Error;

    #[allow(clippy::write_with_newline)]
    fn encode(&mut self, p: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        write!(dst, "{}\r\n", p.typ.to_string())?;
        p.headers
            .iter()
            .map(|(k, v)| write!(dst, "{}: {}\r\n", k.to_uppercase(), v))
            .collect::<Result<(), std::fmt::Error>>()?;
        write!(dst, "\r\n")?;
        Ok(())
    }
}
