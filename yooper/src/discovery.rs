use std::net::{Ipv4Addr, SocketAddr};
use std::collections::HashMap;

use tokio::{
    select,
    net::UdpSocket,
    time::{self, Duration},
    stream::StreamExt,
};
use tokio_util::udp::UdpFramed;
use uuid::{Uuid, self};
use mac_address::{get_mac_address, MacAddressError};
use os_info;
use std::time::{SystemTime, UNIX_EPOCH};
use futures::sink::SinkExt;

use crate::{
    ssdp::message::{
        Codec, Message, MSearch,

    },
    Error,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const SSDP_ADDRESS: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
const SSDP_PORT: u16 = 1900;

pub struct Discovery {
    uuid: Uuid,
    user_agent: String,
    socket: UdpFramed<Codec>,
}

pub struct Device {
    pub server: String,
    pub address: SocketAddr,
    pub services: Vec<Service>,
}

pub struct Service {
    pub service_name: String,
    pub target: String,
}

impl Discovery {
    pub async fn new() -> Result<Self, Error> {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, SSDP_PORT)).await?;
        socket.join_multicast_v4(SSDP_ADDRESS, Ipv4Addr::UNSPECIFIED)?;
        socket.set_multicast_ttl_v4(4)?;

        Self::from_socket(socket)
    }

    pub fn from_socket(socket: UdpSocket) -> Result<Self, Error> {
        Ok(Self {
            socket: UdpFramed::new(socket, Codec::new()),
            uuid: get_uuid()?,
            user_agent: user_agent(),
        })
    }

    pub async fn start_search(&mut self, secs: u8) -> Result<(), Error> {
        // TODO: secs should be between 1 and 5
        let msg = Message::MSearch(
            MSearch {
                max_wait: Some(secs),
                target: "ssdp:all".into(),
                user_agent: Some(self.user_agent.clone()),
                host: format!("{}:{}", SSDP_ADDRESS, SSDP_PORT),

                friendly_name: None, //Some("yooper".into()),
                uuid: None, // Some(self.uuid.to_string()),

                ..Default::default()
            }
        );

        self.socket.send((msg, (SSDP_ADDRESS, SSDP_PORT).into())).await?;

        Ok(())
    }

    pub async fn find(&mut self, secs: u8) -> Result<Vec<Device>, Error> {
        let mut map: HashMap<SocketAddr, Device> = HashMap::new();
        self.start_search(secs).await?;

        let mut delay = time::delay_for(Duration::from_secs(secs.into()));

        loop {
        select!{
            msg = self.socket.next() => {
                match msg {
                    Some(Err(e)) => eprintln!("Error receiving: {:?}", e),
                    Some(Ok((Message::SearchResponse(sr), address))) => {
                        let device = map.entry(address).or_insert(Device {
                            address,
                            server: sr.server,
                            services: Vec::new(),

                        });
                        device.services.push(Service{
                            target: sr.target,
                            service_name: sr.unique_service_name
                        })
                    }
                    _ => (),
                }
            }
            _ = &mut delay => {
                break
            }
        };
        }

        Ok(map.into_iter().map(|(_k, v)| v ).collect())
    }
}

fn user_agent() -> String {
    let info = os_info::get();

    format!("{}/{}.1 upnp/2.0 yooper/{}", info.os_type(), info.version(), VERSION)
}

fn get_uuid() -> Result<Uuid, Error> {
    let mac = get_mac_address()?.ok_or(MacAddressError::InternalError)?;
    let ctx = uuid::v1::Context::new(0);

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ts = uuid::v1::Timestamp::from_unix(&ctx, since_the_epoch.as_secs(), since_the_epoch.subsec_nanos());

    Ok(uuid::Uuid::new_v1(ts, &mac.bytes())?)
}
