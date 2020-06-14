//! Discovery is the the first step of UPnP.
//! Using multicast, ask all devices on the network to announce themselves.
//! From this list, you can then [Describe them](../description) to find out more about their capabilities
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};

use futures::sink::SinkExt;
use mac_address::{get_mac_address, MacAddressError};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::{
    net::UdpSocket,
    select,
    stream::StreamExt,
    time::{self, Duration},
};
use tokio_util::udp::UdpFramed;
use uuid::{self, Uuid};

use crate::{
    ssdp::message::{Codec, MSearch, Message, SearchTarget, UniqueServiceName},
    Error,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const SSDP_ADDRESS: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
const SSDP_PORT: u16 = 1900;

/// Discover services on your network
pub struct Discovery {
    uuid: Uuid,
    user_agent: String,
    socket: UdpFramed<Codec>,
}

/// A Device that's responded to a search
pub struct Device {
    /// version information for the server that responded to the search
    pub server: String,
    /// The address the device responded from
    pub address: SocketAddr,
    /// A list of discovered services
    pub services: Vec<Service>,
    /// the location to retrieve more service information
    pub location: String,
}

/// A Service represents a running service on a device
pub struct Service {
    /// Unique Service Name identifies a unique instance of a device or service.
    pub service_name: UniqueServiceName,
    /// the search target you would use to describe this service
    pub target: SearchTarget,
}

impl Discovery {
    /// Create a new Discovery struct, including creating a new socket
    pub async fn new() -> Result<Self, Error> {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, SSDP_PORT)).await?;
        socket.join_multicast_v4(SSDP_ADDRESS, Ipv4Addr::UNSPECIFIED)?;
        socket.set_multicast_ttl_v4(4)?;

        Self::from_socket(socket)
    }

    /// Create a new Discovery struct based on an existing Tokio socket
    pub fn from_socket(socket: UdpSocket) -> Result<Self, Error> {
        Ok(Self {
            socket: UdpFramed::new(socket, Codec::new()),
            uuid: get_uuid()?,
            user_agent: user_agent(),
        })
    }

    /// Send out an MSearch packet to discover services
    pub async fn start_search(&mut self, secs: u8) -> Result<(), Error> {
        // TODO: secs should be between 1 and 5
        let msg = Message::MSearch(MSearch {
            max_wait: Some(secs),
            target: SearchTarget::All,
            user_agent: Some(self.user_agent.clone()),
            host: format!("{}:{}", SSDP_ADDRESS, SSDP_PORT),

            friendly_name: Some("yooper".into()),
            uuid: Some(self.uuid.to_string()),

            ..Default::default()
        });

        self.socket
            .send((msg, (SSDP_ADDRESS, SSDP_PORT).into()))
            .await?;

        Ok(())
    }

    /// Find all SSDP services on the network.
    /// Will block for n secs then return a list of discovered devices
    /// secs should be between 1 and 5 to comply with
    pub async fn find(&mut self, secs: u8) -> Result<Vec<Device>, Error> {
        let mut map: HashMap<String, Device> = HashMap::new();
        self.start_search(secs).await?;

        let mut delay = time::delay_for(Duration::from_secs(secs.into()));

        loop {
            select! {
                msg = self.socket.next() => {
                    match msg {
                        Some(Err(e)) => eprintln!("Error receiving: {:?}", e),
                        Some(Ok((Message::SearchResponse(sr), address))) => {
                            let uuid = sr.unique_service_name.uuid.clone();
                            map.entry(uuid).or_insert(Device{
                                server: sr.server,
                                address,
                                services: Vec::new(),
                                location: sr.secure_location.unwrap_or(sr.location),
                            }).services.push(Service{
                                target: sr.target,
                                service_name: sr.unique_service_name,
                            });
                        }
                        _ => (),
                    }
                }
                _ = &mut delay => {
                    break
                }
            };
        }

        Ok(map.into_iter().map(|(_k, v)| v).collect())
    }
}

fn user_agent() -> String {
    let info = os_info::get();

    format!(
        "{}/{}.1 upnp/2.0 yooper/{}",
        info.os_type(),
        info.version(),
        VERSION
    )
}

fn get_uuid() -> Result<Uuid, Error> {
    let mac = get_mac_address()?.ok_or(MacAddressError::InternalError)?;
    let ctx = uuid::v1::Context::new(0);

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ts = uuid::v1::Timestamp::from_unix(
        &ctx,
        since_the_epoch.as_secs(),
        since_the_epoch.subsec_nanos(),
    );

    Ok(uuid::Uuid::new_v1(ts, &mac.bytes())?)
}
