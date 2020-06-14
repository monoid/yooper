#[cfg(test)]
mod tests;

use crate::Error;
use serde::{Deserialize, Deserializer};
use serde_with::rust::display_fromstr;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct DeviceType {
    vendor_domain: Option<String>,
    device_type: String,
    version: String,
}

impl ToString for DeviceType {
    fn to_string(&self) -> String {
        format!(
            "urn:{}:device:{}:{}",
            self.vendor_domain
                .as_ref()
                .map_or("schemas-upnp-org", String::as_ref),
            self.device_type,
            self.version,
        )
    }
}

impl FromStr for DeviceType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(':').collect::<Vec<&str>>().as_slice() {
            ["urn", "schemas-upnp-org", "device", device_type, version] => Ok(Self {
                vendor_domain: None,
                device_type: device_type.to_string(),
                version: version.to_string(),
            }),

            ["urn", vendor_domain, "device", device_id, version] => Ok(Self {
                vendor_domain: Some(vendor_domain.to_string()),
                device_type: device_id.to_string(),
                version: version.to_string(),
            }),
            _ => Err(Error::MalformedField("service_id", s.to_owned())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ServiceType {
    vendor_domain: Option<String>,
    service_type: String,
    version: String,
}

impl FromStr for ServiceType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(':').collect::<Vec<&str>>().as_slice() {
            ["urn", "schemas-upnp-org", "service", device_type, version] => Ok(Self {
                vendor_domain: None,
                service_type: device_type.to_string(),
                version: version.to_string(),
            }),

            ["urn", vendor_domain, "service", service_id, version] => Ok(Self {
                vendor_domain: Some(vendor_domain.to_string()),
                service_type: service_id.to_string(),
                version: version.to_string(),
            }),
            _ => Err(Error::MalformedField("service_id", s.to_owned())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ServiceId {
    pub vendor_domain: Option<String>,
    pub service_id: String,
}

impl FromStr for ServiceId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(':').collect::<Vec<&str>>().as_slice() {
            ["urn", "upnp-org", "serviceId", service_id] => Ok(Self {
                vendor_domain: None,
                service_id: service_id.to_string(),
            }),

            ["urn", vendor_domain, "serviceId", service_id] => Ok(Self {
                vendor_domain: Some(vendor_domain.to_string()),
                service_id: service_id.to_string(),
            }),
            _ => Err(Error::MalformedField("service_id", s.to_owned())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UniqueDeviceName {
    pub uuid: String,
}

impl FromStr for UniqueDeviceName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("uuid:") {
            Ok(Self {
                uuid: s[5..].to_owned(),
            })
        } else {
            Err(Error::MalformedField("udn", s.to_owned()))
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    #[serde(with = "display_fromstr", rename = "deviceType")]
    pub device_type: DeviceType,
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
    pub manufacturer: String,
    #[serde(rename = "manufacturerURL")]
    pub manufacturer_url: Option<String>,

    pub model_description: Option<String>,
    pub model_name: Option<String>,
    pub model_number: Option<String>,
    #[serde(rename = "modelURL")]
    pub model_url: Option<String>,

    pub serial_number: Option<String>,
    #[serde(with = "display_fromstr", rename = "UDN")]
    pub unique_device_name: UniqueDeviceName,
    #[serde(rename = "UPC")]
    pub upc: Option<String>,

    // TODO(EKF): IconList
    #[serde(
        rename = "serviceList",
        deserialize_with = "deserialize_services",
        default
    )]
    pub services: Vec<Service>,
    #[serde(
        rename = "deviceList",
        deserialize_with = "deserialize_devices",
        default
    )]
    pub devices: Vec<Device>,

    #[serde(rename = "presentationURL")]
    pub presentation_url: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    #[serde(with = "display_fromstr")]
    pub service_type: ServiceType,
    #[serde(with = "display_fromstr")]
    pub service_id: ServiceId,

    #[serde(rename = "SCPDURL")]
    pub scpd_url: String,

    #[serde(rename = "controlURL")]
    pub control_url: String,
    #[serde(rename = "eventSubURL")]
    pub event_sub_url: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DescriptionDocument {
    pub root: Description,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub config_id: Option<String>,
    pub spec_version: SpecVersion,
    pub device: Device,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SpecVersion {
    pub major: u32,
    pub minor: u32,
}

#[derive(Debug, PartialEq, Deserialize)]
struct ServiceOuter {
    service: Vec<Service>,
}

/// Flatten the `service` list down
fn deserialize_services<'de, D>(d: D) -> Result<Vec<Service>, D::Error>
where
    D: Deserializer<'de>,
{
    ServiceOuter::deserialize(d).map(|s| s.service)
}

#[derive(Debug, PartialEq, Deserialize)]
struct DeviceOuter {
    device: Vec<Device>,
}

/// Flatten the `device` list down
fn deserialize_devices<'de, D>(d: D) -> Result<Vec<Device>, D::Error>
where
    D: Deserializer<'de>,
{
    DeviceOuter::deserialize(d).map(|d| d.device)
}