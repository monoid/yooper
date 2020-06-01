use super::Message;
use crate::ssdp::packet::{self, FromPacket, ToPacket};
use crate::Error;

use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

/// A codec for turning udp packets into decoded Messages
#[derive(Default)]
pub struct Codec {
    encoder: packet::Encoder,
    decoder: packet::Decoder,
}

impl Codec {
    pub fn new() -> Self {
        Codec {
            encoder: packet::Encoder {},
            decoder: packet::Decoder {},
        }
    }
}

impl Encoder<Message> for Codec {
    type Error = Error;

    fn encode(&mut self, p: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        self.encoder.encode(p.to_packet(), dst)
    }
}

impl Decoder for Codec {
    type Item = Message;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.decoder.decode(src) {
            Err(e) => Err(e),
            Ok(None) => Ok(None),
            Ok(Some(v)) => Message::from_packet(&v).map(Some),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        types::{Ext, SearchTarget, UniqueServiceName},
        SearchResponse,
    };
    use super::*;
    use crate::ssdp::tests::constants::*;

    #[test]
    fn test_decode_ok() {
        let mut buf = BytesMut::from(SEARCH_RESPONSE_EXAMPLE);
        let mut decoder = Codec::default();
        let message = Message::SearchResponse(SearchResponse {
            max_age: "max-age=1800".into(),
            date: Some("Mon, 25 May 2020 02:39:02 GMT".into()),
            location: "http://192.168.7.1:1900/igd.xml".into(),
            secure_location: None,
            server: "eeroOS/latest UPnP/1.0 eero/latest".into(),
            target: SearchTarget::UUID("fcdb9233-a63f-41da-b42c-7cfeb99c8adf".parse().unwrap()),
            unique_service_name: UniqueServiceName {
                uuid: "fcdb9233-a63f-41da-b42c-7cfeb99c8adf".into(),
                search_target: None,
            },

            ext: Ext {},
            boot_id: None,
            config_id: None,
            search_port: None,
        });

        assert_eq!(message, decoder.decode(&mut buf).unwrap().unwrap())
    }
}
