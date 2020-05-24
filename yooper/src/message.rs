use crate::packet::FromPacket;

#[derive(Debug, PartialEq, FromPacket, Default)]
pub enum Message {
    #[message(reqline = "MSearch")]
    MSearch {
        /// Maximum wait time in seconds. shall be greater than or equal to 1 and should
        /// be less than 5 inclusive.
        max_wait: u8,
        /// Field value contains Search Target.
        // TODO: enum
        target: String,
        /// Field value shall begin with the following “product tokens” (defined
        /// by HTTP/1.1). The first product token identifes the operating system in the form OS name/OS version, the
        /// second token represents the UPnP version and shall be UPnP/2.0, and the third token identifes the product
        /// using the form product name/product version. For example, “USER-AGENT: unix/5.1 UPnP/2.0
        /// MyProduct/1.0”.
        user_agent: Option<String>,
        /// control point can request that a device replies to a TCP port on the control point. When this header
        /// is present it identifies the TCP port on which the device can reply to the search.
        tcp_port: Option<u16>,
        /// Specifies the friendly name of the control point. The friendly name is vendor specific.
        friendly_name: String,
        /// uuid of the control point. When the control point is implemented in a UPnP device it is recommended
        /// to use the UDN of the co-located UPnP device. When implemented, all specified requirements for uuid usage
        /// in devices also apply for control points.
        uuid: Option<String>,
    },
    #[message(reqline = "Notify", sts = "ssdp:alive")]
    Available {
        /// after this duration, control points should assume the device (or
        /// service)  is  no  longer  available;  as  long  as  a  control  point
        /// has  received  at  least  one  advertisement  that  is  still  valid
        /// from a root device, any of its embedded devices or any of its
        ///services, then the control point can assume that  all  are  available.
        max_age: u32,
        /// Field  value  contains  a  URL  to  the  UPnP  description  of  the
        /// root  device.  Normally  the  host  portion  contains  a  literal
        /// IP  address  rather  than  a  domain  name  in  unmanaged  networks.
        /// Specified  by  UPnP  vendor. Single absolute URL (see RFC 3986)
        location: String,

        secure_location: Option<String>,
        // TODO: Enum
        notification_type: String,
        /// Field  value  shall begin  with  the  following  “product  tokens”
        /// (defined by HTTP/1.1). The first product token identifes the operating
        /// system in the form OSname/OSversion, the  second  token  represents
        /// the  UPnP version  and  shall be UPnP/2.0,  and  the  third  token
        /// identifes the product using the form productname/productversion.
        server: String,

        /// Identifies a unique instance of a device or service.
        // TODO: Enum
        unique_service_name: String,
        /// presents the boot instance of the device expressed according to a monotonically increasing value.
        boot_id: i32,

        config_id: i32,

        search_port: Option<u16>,
    },
    Unimplemented,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::{Packet, PacketType::*};
    #[test]
    fn test_available_from_packet() {
        let packet = Packet::new_from_literal(Notify, vec![("sts", "ssdp:available")]);
        let expected = Message::Available {
            ..Default::default()
        };

        assert_eq!(expected, Message::from_packet(&packet).unwrap())
    }
}
