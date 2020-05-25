use crate::{FromPacket, ToPacket};

#[derive(Debug, PartialEq, FromPacket, ToPacket)]
pub enum Message {
    // #[message(reqline = "MSearch")]
    // MSearch {
    //     /// Maximum wait time in seconds. shall be greater than or equal to 1 and should
    //     /// be less than 5 inclusive.
    //     #[header("cache-control")]
    //     max_wait: String,
    //     /// Field value contains Search Target.
    //     // TODO: enum
    //     target: String,
    //     /// Field value shall begin with the following “product tokens” (defined
    //     /// by HTTP/1.1). The first product token identifes the operating system in the form OS name/OS version, the
    //     /// second token represents the UPnP version and shall be UPnP/2.0, and the third token identifes the product
    //     /// using the form product name/product version. For example, “USER-AGENT: unix/5.1 UPnP/2.0
    //     /// MyProduct/1.0”.
    //     user_agent: Option<String>,
    //     /// control point can request that a device replies to a TCP port on the control point. When this header
    //     /// is present it identifies the TCP port on which the device can reply to the search.
    //     tcp_port: Option<u16>,
    //     /// Specifies the friendly name of the control point. The friendly name is vendor specific.
    //     friendly_name: String,
    //     /// uuid of the control point. When the control point is implemented in a UPnP device it is recommended
    //     /// to use the UDN of the co-located UPnP device. When implemented, all specified requirements for uuid usage
    //     /// in devices also apply for control points.
    //     uuid: Option<String>,
    // },
    #[message(reqline = "Notify", nts = "ssdp:alive")]
    Available {
        /// after this duration, control points should assume the device (or
        /// service)  is  no  longer  available;  as  long  as  a  control  point
        /// has  received  at  least  one  advertisement  that  is  still  valid
        /// from a root device, any of its embedded devices or any of its
        ///services, then the control point can assume that  all  are  available.
        #[header("cache-control")]
        max_age: String,
        /// Field  value  contains  a  URL  to  the  UPnP  description  of  the
        /// root  device.  Normally  the  host  portion  contains  a  literal
        /// IP  address  rather  than  a  domain  name  in  unmanaged  networks.
        /// Specified  by  UPnP  vendor. Single absolute URL (see RFC 3986)
        location: String,

        host: String,

        #[header("securelocation.upnp.org")]
        secure_location: Option<String>,
        // TODO: Enum
        #[header("nt")]
        notification_type: String,
        /// Field  value  shall begin  with  the  following  “product  tokens”
        /// (defined by HTTP/1.1). The first product token identifes the operating
        /// system in the form OSname/OSversion, the  second  token  represents
        /// the  UPnP version  and  shall be UPnP/2.0,  and  the  third  token
        /// identifes the product using the form productname/productversion.
        server: String,

        /// Identifies a unique instance of a device or service.
        // TODO: Enum
        #[header("usn")]
        unique_service_name: String,
        /// presents the boot instance of the device expressed according to a monotonically increasing value.
        #[header("bootid.upnp.org")]
        boot_id: Option<i32>,

        #[header("configid.upnp.org")]
        config_id: Option<i32>,
        #[header("searchport.upnp.org")]
        search_port: Option<u16>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::{FromPacket, Packet, PacketType::*, ToPacket};

    fn packet() -> Packet {
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
        Message::Available {
            max_age: "max-age=3600".into(),
            location: "http://192.168.7.238:54216/RootDevice.xml".into(),
            notification_type: "urn:schemas-upnp-org:device:MediaServer:1".into(),
            server: "Windows 10/10.0 UPnP/1.0 Azureus/5.7.6.0".into(),
            unique_service_name: "uuid:07853410-ccef-9e3c-de6a-410b371182eb::urn:schemas-upnp-org:device:MediaServer:1".into(),
            host:  "239.255.255.250:1900".into(),

            secure_location: None,
            boot_id: None,
            config_id: None,
            search_port: Some(11120),
        }
    }

    #[test]
    fn test_available_from_packet() {
        let packet = packet();
        let expected = available();
        assert_eq!(expected, Message::from_packet(&packet).unwrap())
    }

    #[test]
    fn test_packet_from_availabe() {
        let available = available();
        let expected = packet();
        assert_eq!(expected, available.to_packet())
    }
}
