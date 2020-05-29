//! A set of symbolic representations of SSDP packets

mod codec;
pub(self) mod types;

use crate::ssdp::packet::{FromHeaders, FromPacket, ToHeaders, ToPacket};
pub use codec::Codec;

pub use types::SearchTarget;

#[derive(ToHeaders, FromHeaders, Debug, PartialEq, Default)]
pub struct MSearch {
    pub host: String,

    pub man: types::ManDiscover,

    #[header("cache-control")]
    pub cache_control: Option<String>,

    /// Maximum wait time in seconds. shall be greater than or equal to 1 and should
    /// be less than 5 inclusive.
    #[header("mx")]
    pub max_wait: Option<u8>,

    // TODO: enum
    /// Field value contains Search Target.
    #[header("st")]
    pub target: types::SearchTarget,
    /// Field value shall begin with the following “product tokens” (defined
    /// by HTTP/1.1). The first product token identifes the operating system in the form OS name/OS version, the
    /// second token represents the UPnP version and shall be UPnP/2.0, and the third token identifes the product
    /// using the form product name/product version. For example, “USER-AGENT: unix/5.1 UPnP/2.0
    /// MyProduct/1.0”.
    pub user_agent: Option<String>,
    /// if set, this TCP port can be used for any follow up requests
    #[header("tcpport.upnp.org")]
    pub tcp_port: Option<u16>,
    /// Specifies the friendly name of the control point. The friendly name is vendor specific.
    #[header("cpfn.upnp.org")]
    pub friendly_name: Option<String>,
    /// uuid of the control point.
    #[header("cpuuid.upnp.org")]
    pub uuid: Option<String>,
}

#[derive(ToHeaders, FromHeaders, Debug, PartialEq)]
pub struct Available {
    pub host: String,

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

    /// A number identifying this particular configuration.
    /// if configuration changes, this should change as well
    #[header("configid.upnp.org")]
    pub config_id: Option<i32>,
    /// A port other than 1900 than can be used for queries
    #[header("searchport.upnp.org")]
    pub search_port: Option<u16>,
}

#[derive(ToHeaders, FromHeaders, Debug, PartialEq)]
pub struct SearchResponse {
    /// Specifies how long this response is valid
    #[header("cache-control")]
    pub max_age: String,

    /// When the responce was generated
    pub date: Option<String>,

    /// The URL for the UPNP description of the root device
    pub location: String,

    ext: types::Ext,

    /// A server string like "unix/5.1 UPnP/2.0 MyProduct/1.0"
    pub server: String,

    /// If set, a base url with https:// that can be used instead of location
    #[header("securelocation.upnp.org")]
    pub secure_location: Option<String>,

    #[header("st")]
    pub target: types::SearchTarget,

    /// A unique service name for this particular service
    // TODO: Enum
    #[header("usn")]
    pub unique_service_name: String,

    /// presents the boot instance of the device expressed according to a monotonically increasing value.
    #[header("bootid.upnp.org")]
    pub boot_id: Option<i32>,

    /// A number identifying this particular configuration.
    /// if configuration changes, this should change as well
    #[header("configid.upnp.org")]
    pub config_id: Option<i32>,
    /// A port other than 1900 than can be used for queries
    #[header("searchport.upnp.org")]
    pub search_port: Option<u16>,
}

/// Any SSDP message
#[derive(Debug, PartialEq, FromPacket, ToPacket)]
pub enum Message {
    /// Search the network for other devices
    #[message(reqline = "MSearch")]
    MSearch(MSearch),
    /// Notification that a device has been added to the network
    #[message(reqline = "Notify", nts = "ssdp:alive")]
    Available(Available),
    /// A response to a search query
    #[message(reqline = "Ok")]
    SearchResponse(SearchResponse),
}

#[cfg(test)]
mod tests;
