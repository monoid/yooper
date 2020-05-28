mod codec;
pub(self) mod types;

use crate::ssdp::packet::{FromHeaders, FromPacket, ToHeaders, ToPacket};
pub use codec::Codec;

#[derive(ToHeaders, FromHeaders, Debug, PartialEq)]
pub struct MSearch {
    /// Maximum wait time in seconds. shall be greater than or equal to 1 and should
    /// be less than 5 inclusive.
    #[header("cache-control")]
    pub max_wait: Option<String>,
    /// Field value contains Search Target.
    // TODO: enum
    #[header("st")]
    pub target: String,
    /// Field value shall begin with the following “product tokens” (defined
    /// by HTTP/1.1). The first product token identifes the operating system in the form OS name/OS version, the
    /// second token represents the UPnP version and shall be UPnP/2.0, and the third token identifes the product
    /// using the form product name/product version. For example, “USER-AGENT: unix/5.1 UPnP/2.0
    /// MyProduct/1.0”.
    pub user_agent: Option<String>,
    /// control point can request that a device replies to a TCP port on the control point. When this header
    /// is present it identifies the TCP port on which the device can reply to the search.
    #[header("tcpport.upnp.org")]
    pub tcp_port: Option<u16>,
    /// Specifies the friendly name of the control point. The friendly name is vendor specific.
    #[header("cpfn.upnp.org")]
    pub friendly_name: Option<String>,
    /// uuid of the control point. When the control point is implemented in a UPnP device it is recommended
    /// to use the UDN of the co-located UPnP device. When implemented, all specified requirements for uuid usage
    /// in devices also apply for control points.
    #[header("cpuuid.upnp.org")]
    pub uuid: Option<String>,
}

#[derive(ToHeaders, FromHeaders, Debug, PartialEq)]
pub struct Available {
    /// after this duration, control points should assume the device (or
    /// service)  is  no  longer  available;  as  long  as  a  control  point
    /// has  received  at  least  one  advertisement  that  is  still  valid
    /// from a root device, any of its embedded devices or any of its
    ///services, then the control point can assume that  all  are  available.
    #[header("cache-control")]
    pub max_age: String,
    /// Field  value  contains  a  URL  to  the  UPnP  description  of  the
    /// root  device.  Normally  the  host  portion  contains  a  literal
    /// IP  address  rather  than  a  domain  name  in  unmanaged  networks.
    /// Specified  by  UPnP  vendor. Single absolute URL (see RFC 3986)
    pub location: String,

    pub host: String,

    #[header("securelocation.upnp.org")]
    pub secure_location: Option<String>,
    // TODO: Enum
    #[header("nt")]
    pub notification_type: String,
    /// Field  value  shall begin  with  the  following  “product  tokens”
    /// (defined by HTTP/1.1). The first product token identifes the operating
    /// system in the form OSname/OSversion, the  second  token  represents
    /// the  UPnP version  and  shall be UPnP/2.0,  and  the  third  token
    /// identifes the product using the form productname/productversion.
    pub server: String,

    /// Identifies a unique instance of a device or service.
    // TODO: Enum
    #[header("usn")]
    pub unique_service_name: String,
    /// presents the boot instance of the device expressed according to a monotonically increasing value.
    #[header("bootid.upnp.org")]
    pub boot_id: Option<i32>,

    #[header("configid.upnp.org")]
    pub config_id: Option<i32>,
    #[header("searchport.upnp.org")]
    pub search_port: Option<u16>,
}

#[derive(ToHeaders, FromHeaders, Debug, PartialEq)]
pub struct SearchResponse {
    #[header("cache-control")]
    pub max_age: String,

    pub date: Option<String>,
    pub location: String,

    ext: types::Ext,

    #[header("securelocation.upnp.org")]
    pub secure_location: Option<String>,
    pub server: String,

    // TODO: enum
    #[header("st")]
    pub target: String,

    // TODO: Enum
    #[header("usn")]
    pub unique_service_name: String,

    /// presents the boot instance of the device expressed according to a monotonically increasing value.
    #[header("bootid.upnp.org")]
    pub boot_id: Option<i32>,

    #[header("configid.upnp.org")]
    pub config_id: Option<i32>,
    #[header("searchport.upnp.org")]
    pub search_port: Option<u16>,
}

#[derive(Debug, PartialEq, FromPacket, ToPacket)]
pub enum Message {
    #[message(reqline = "MSearch")]
    MSearch(MSearch),
    #[message(reqline = "Notify", nts = "ssdp:alive")]
    Available(Available),
    #[message(reqline = "Ok")]
    SearchResponse(SearchResponse),
}

#[cfg(test)]
mod tests;
