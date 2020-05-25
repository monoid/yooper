use super::Message;
use crate::{Error, FromPacket, SSDPDecoder, SSDPEncoder, ToPacket};

use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

#[derive(Default)]
pub struct SSDPMessageDecoder {
    inner: SSDPDecoder,
}

impl Decoder for SSDPMessageDecoder {
    type Item = Message;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.inner.decode(src) {
            Err(e) => Err(e),
            Ok(None) => Ok(None),
            Ok(Some(v)) => Message::from_packet(&v).map(Some),
        }
    }
}

#[derive(Default)]
pub struct SSDPMessageEncoder {
    inner: SSDPEncoder,
}

impl Encoder<Message> for SSDPMessageEncoder {
    type Error = Error;

    fn encode(&mut self, p: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        self.inner.encode(p.to_packet(), dst)
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::Ext;
    use super::*;
    use crate::tests::constants::*;

    #[test]
    fn test_decode_ok() {
        let mut buf = BytesMut::from(SEARCH_RESPONSE_EXAMPLE);
        let mut decoder = SSDPMessageDecoder::default();
        let message = Message::SearchResponse {
            max_age: "max-age=1800".into(),
            date: Some("Mon, 25 May 2020 02:39:02 GMT".into()),
            location: "http://192.168.7.1:1900/igd.xml".into(),
            secure_location: None,
            server: "eeroOS/latest UPnP/1.0 eero/latest".into(),
            target: "uuid:fcdb9233-a63f-41da-b42c-7cfeb99c8adf".into(),
            unique_service_name: "uuid:fcdb9233-a63f-41da-b42c-7cfeb99c8adf".into(),

            ext: Ext {},
            boot_id: None,
            config_id: None,
            search_port: None,
        };

        assert_eq!(message, decoder.decode(&mut buf).unwrap().unwrap())
    }
}
