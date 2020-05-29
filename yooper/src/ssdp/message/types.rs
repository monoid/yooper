use crate::Error;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Ext;

impl ToString for Ext {
    fn to_string(&self) -> String {
        String::new()
    }
}

impl FromStr for Ext {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self {}),
            _ => Err(Error::IncorrectHeader("ext")),
        }
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct ManDiscover;

impl ToString for ManDiscover {
    fn to_string(&self) -> String {
        String::from("\"ssdp:discover\"")
    }
}

impl FromStr for ManDiscover {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ssdp:discover" | "\"ssdp:discover\"" => Ok(Self {}),
            _ => Err(Error::IncorrectHeader("man")),
        }
    }
}

/// What kind of control point to search for
#[derive(PartialEq, Debug)]
pub enum SearchTarget {
    /// Search for all devices and services
    All,
    /// Search for root devices only
    RootDevice,
    /// Search for a particular device
    UUID(uuid::Uuid),

    /// Search for any device of this type, where device_type is defined by the UPnP forum
    Device {
        device_type: String,
        version: String,
    },
    /// Search for any service of this type, where service_type is defined by the UPnP forum
    Service {
        service_type: String,
        version: String,
    },

    /// Search for for any device of this type, where device_type is defined by a vendor
    VendorDevice {
        domain_name: String,
        device_type: String,
        version: String,
    },
    /// Search for for any service of this type, where service_type is defined by a vendor
    VendorService {
        domain_name: String,
        service_type: String,
        version: String,
    },

    /// Not everyone plays by the rules. A catch-all for non-standard search types
    Other(String),
}

impl ToString for SearchTarget {
    fn to_string(&self) -> std::string::String {
        use SearchTarget::*;

        match self {
            All => "ssdp:all".to_string(),
            RootDevice => "upnp:rootdevice".to_string(),
            UUID(uuid) => format!("uuid:{}", uuid.to_string()),
            Device {
                device_type,
                version,
            } => format!("urn:schemas-upnp-org:device:{}:{}", device_type, version),
            Service {
                service_type,
                version,
            } => format!("urn:schemas-upnp-org:service:{}:{}", service_type, version),
            VendorDevice {
                domain_name,
                device_type,
                version,
            } => format!("urn:{}:device:{}:{}", domain_name, device_type, version),
            VendorService {
                domain_name,
                service_type,
                version,
            } => format!("urn:{}:sercvice:{}:{}", domain_name, service_type, version),
            Other(s) => s.to_string(),
        }
    }
}

impl FromStr for SearchTarget {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SearchTarget::*;

        Ok(match s.split(':').collect::<Vec<&str>>().as_slice() {
            ["ssdp", "all"] => All,
            ["upnp", "rootdevice"] => RootDevice,
            ["uuid", uuid] => UUID(uuid::Uuid::parse_str(uuid)?),
            ["urn", "schemas-upnp-org", "device", dt, v] => Device {
                device_type: dt.to_string(),
                version: v.to_string(),
            },
            ["urn", "schemas-upnp-org", "service", st, v] => Service {
                service_type: st.to_string(),
                version: v.to_string(),
            },
            ["urn", dn, "device", dt, v] => VendorDevice {
                domain_name: dn.to_string(),
                device_type: dt.to_string(),
                version: v.to_string(),
            },
            ["urn", dn, "service", st, v] => VendorService {
                domain_name: dn.to_string(),
                service_type: st.to_string(),
                version: v.to_string(),
            },
            _ => Other(s.to_owned()),
        })
    }
}

impl Default for SearchTarget {
    fn default() -> Self {
        Self::All
    }
}
