use super::*;
use crate::ssdp::packet::{FromPacket, Packet, PacketType::*, ToPacket};

fn available_packet() -> Packet {
    Packet::new_from_literal(
            Notify,
            vec![("host", "239.255.255.250:1900"),
                 ("cache-control", "max-age=3600"),
                 ("location", "http://192.168.7.238:54216/RootDevice.xml"),
                 ("nt","urn:schemas-upnp-org:device:MediaServer:1"),
                 ("nts", "ssdp:alive"),
                 ("server","Windows 10/10.0 UPnP/1.0 Azureus/5.7.6.0"),
                 ("usn","uuid:07853410-ccef-9e3c-de6a-410b371182eb::urn:schemas-upnp-org:device:MediaServer:1" ),
                 ("searchport.upnp.org", "11120"),
            ]
        )
}

fn available() -> Message {
    let st = SearchTarget::Device {
        device_type: "MediaServer".to_string(),
        version: "1".to_string(),
    };

    Message::Available(Available {
        max_age: "max-age=3600".into(),
        location: "http://192.168.7.238:54216/RootDevice.xml".into(),
        notification_type: st.clone(),
        server: "Windows 10/10.0 UPnP/1.0 Azureus/5.7.6.0".into(),
        unique_service_name: UniqueServiceName {
            uuid: "07853410-ccef-9e3c-de6a-410b371182eb".to_string(),
            search_target: Some(st),
        },
        host: "239.255.255.250:1900".into(),

        secure_location: None,
        boot_id: None,
        config_id: None,
        search_port: Some(11120),
    })
}

#[test]
fn test_available_from_packet() {
    let packet = available_packet();
    let expected = available();
    assert_eq!(expected, Message::from_packet(&packet).unwrap())
}

#[test]
fn test_packet_from_availabe() {
    let available = available();
    let expected = available_packet();
    assert_eq!(expected, available.to_packet())
}
