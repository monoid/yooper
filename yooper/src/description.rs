//! Description is the second step of UPnP, after [Discovery](../Discovery).
//! Using the location retrieved from discovery, retrieve an XML document over HTTP.
//! This document enumerates the capabilities of the given device.
#[cfg(test)]
mod tests;

use crate::Error;
use serde::{Deserialize, Deserializer};
use serde_with::rust::display_fromstr;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct DeviceType {
    /// Will be None for standard devices specified by the UPnP Forum.
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
                device_type: (*device_type).to_owned(),
                version: (*version).to_owned(),
            }),

            ["urn", vendor_domain, "device", device_id, version] => Ok(Self {
                vendor_domain: Some((*vendor_domain).to_owned()),
                device_type: (*device_id).to_owned(),
                version: (*version).to_owned(),
            }),
            _ => Err(Error::MalformedField("service_id", s.to_owned())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ServiceType {
    /// Will be None for standard services specified by the UPnP Forum.
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
                service_type: (*device_type).to_string(),
                version: (*version).to_string(),
            }),

            ["urn", vendor_domain, "service", service_id, version] => Ok(Self {
                vendor_domain: Some((*vendor_domain).to_string()),
                service_type: (*service_id).to_string(),
                version: (*version).to_string(),
            }),
            _ => Err(Error::MalformedField("service_id", s.to_owned())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ServiceId {
    /// Will be None for standard services specified by the UPnP Forum
    pub vendor_domain: Option<String>,
    pub service_id: String,
}

impl FromStr for ServiceId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(':').collect::<Vec<&str>>().as_slice() {
            ["urn", "upnp-org", "serviceId", service_id] => Ok(Self {
                vendor_domain: None,
                service_id: (*service_id).to_string(),
            }),

            ["urn", vendor_domain, "serviceId", service_id] => Ok(Self {
                vendor_domain: Some((*vendor_domain).to_string()),
                service_id: (*service_id).to_string(),
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

/// A Logical device.
/// One physical "Device" may contain multiple logical Devices.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    /// UPnP Device Type
    #[serde(with = "display_fromstr", rename = "deviceType")]
    pub device_type: DeviceType,
    /// Short description for end user. Should be < 64 characters.
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
    /// Manufacturer's name. Should be <64 characters.
    pub manufacturer: String,
    ///Web site for Manufacturer
    #[serde(rename = "manufacturerURL")]
    pub manufacturer_url: Option<String>,

    /// Long description for the end user. Should be < 128 characters
    pub model_description: Option<String>,
    /// Should be < 128 characters
    pub model_name: Option<String>,
    /// Should be < 32 characters
    pub model_number: Option<String>,
    #[serde(rename = "modelURL")]
    pub model_url: Option<String>,

    /// Shourd be < 64 characters
    pub serial_number: Option<String>,
    /// universally-unique identifier for the device.
    /// Shall be the same over time for a specific device.
    /// Shall max the field value of the NT header in discovery messages
    #[serde(with = "display_fromstr", rename = "UDN")]
    pub unique_device_name: UniqueDeviceName,
    #[serde(rename = "UPC")]
    /// Universal product code.
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
    /// A page to display to the end user
    pub presentation_url: Option<String>,
}

/// Logical functional unit, Smallest  units of control.
/// Exposes actions and models the state of a physical device with state variables.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    #[serde(with = "display_fromstr")]
    pub service_type: ServiceType,
    #[serde(with = "display_fromstr")]
    pub service_id: ServiceId,

    /// URL for service description. Relative to device description URL
    #[serde(rename = "SCPDURL")]
    pub scpd_url: String,

    /// Relative to device description URL
    #[serde(rename = "controlURL")]
    pub control_url: String,
    /// Relative to device description URL
    #[serde(rename = "eventSubURL")]
    pub event_sub_url: String,
}

/// This document contains the root device description and metadata
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

/// Retrieve and parse a device description.
/// See the location field from [discovery::Device](../discovery/struct.Device.html#structfield.location).
pub async fn describe(location: String) -> Result<Device, Error> {
    let body = reqwest::get(&location).await?.text().await?;

    let document: Description = serde_xml_rs::from_str(&body)?;
    Ok(document.device)
}
