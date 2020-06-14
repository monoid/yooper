use super::*;

// Harvested from my wireless router
const IGD_EXAMPLE: &str = include_str!("testdata/igd.xml");

#[test]
fn test_device_type_from_str() {
    let s = "urn:schemas-upnp-org:device:deviceType:ver";
    assert_eq!(
        DeviceType {
            vendor_domain: None,
            device_type: "deviceType".into(),
            version: "ver".into()
        },
        s.parse().unwrap()
    );

    let s2 = "urn:domain-name:device:deviceType:ver";
    assert_eq!(
        DeviceType {
            vendor_domain: Some("domain-name".into()),
            device_type: "deviceType".into(),
            version: "ver".into()
        },
        s2.parse().unwrap()
    );

    let s3 = "urn:non-matching:service:value";
    match s3.parse::<DeviceType>().unwrap_err() {
        Error::MalformedField("service_id", v) if v == s3 => (),
        e => panic!("Didn't get the error we assumed! {:?}", e),
    };
}

#[test]
fn test_deserialize_with_example() {
    let expected = Description {
        config_id: None,
        spec_version: SpecVersion { major: 1, minor: 0 },
        device: Device {
            device_type: DeviceType {
                vendor_domain: None,
                device_type: "InternetGatewayDevice".into(),
                version: "1".into(),
            },
            friendly_name: "".into(),
            manufacturer: "".into(),
            manufacturer_url: Some("".into()),

            model_description: Some("".into()),
            model_name: Some("".into()),
            model_number: Some("".into()),
            model_url: None,

            serial_number: None,
            unique_device_name: UniqueDeviceName { uuid: "".into() },
            upc: None,
            services: vec![Service {
                service_type: ServiceType {
                    vendor_domain: None,
                    service_type: "Layer3Forwarding".into(),
                    version: "1".into(),
                },
                service_id: ServiceId {
                    vendor_domain: None,
                    service_id: "L3Forwarding1".into(),
                },
                scpd_url: "/l3f.xml".into(),
                control_url: "/l3f".into(),
                event_sub_url: "/l3f/events".into(),
            }],

            devices: vec![Device {
                device_type: DeviceType {
                    vendor_domain: None,
                    device_type: "WANDevice".into(),
                    version: "1".into(),
                },
                friendly_name: "".into(),
                manufacturer: "".into(),
                manufacturer_url: Some("".into()),

                model_description: Some("".into()),
                model_name: Some("".into()),
                model_number: Some("".into()),
                model_url: None,

                serial_number: None,
                unique_device_name: UniqueDeviceName { uuid: "".into() },
                upc: None,
                services: vec![Service {
                    service_type: ServiceType {
                        vendor_domain: None,
                        service_type: "WANCommonInterfaceConfig".into(),
                        version: "1".into(),
                    },
                    service_id: ServiceId {
                        vendor_domain: None,
                        service_id: "WANCommonInterfaceConfig".into(),
                    },
                    scpd_url: "/ifc.xml".into(),
                    control_url: "/ifc".into(),
                    event_sub_url: "/ifc/events".into(),
                }],

                devices: vec![Device {
                    device_type: DeviceType {
                        vendor_domain: None,
                        device_type: "WANConnectionDevice".into(),
                        version: "1".into(),
                    },
                    friendly_name: "".into(),
                    manufacturer: "".into(),
                    manufacturer_url: Some("".into()),

                    model_description: Some("".into()),
                    model_name: Some("".into()),
                    model_number: Some("".into()),
                    model_url: None,

                    serial_number: None,
                    unique_device_name: UniqueDeviceName { uuid: "".into() },
                    upc: None,
                    services: vec![Service {
                        service_type: ServiceType {
                            vendor_domain: None,
                            service_type: "WANIPConnection".into(),
                            version: "1".into(),
                        },
                        service_id: ServiceId {
                            vendor_domain: None,
                            service_id: "WANIPConnection".into(),
                        },
                        scpd_url: "/ipc.xml".into(),
                        control_url: "/ipc".into(),
                        event_sub_url: "/ipc/events".into(),
                    }],
                    devices: vec![],
                    presentation_url: None,
                }],
                presentation_url: None,
            }],
            presentation_url: Some("".into()),
        },
    };

    assert_eq!(expected, serde_xml_rs::from_str(IGD_EXAMPLE).unwrap());
}
