#![cfg(test)]
use super::*;

const NOTIFY_EXAMPLE: &[u8] = include_bytes!("notify.bin");

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
