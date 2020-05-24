use std::error::Error;
use std::net::Ipv4Addr;
use tokio::net::UdpSocket;

// const VERSION: &'static str = env!("CARGO_PKG_VERSION");
// const OS: &'static str = "linux"; //TODO
const SSDP_ADDRESS: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
const SSDP_PORT: u16 = 1900;

const WLAN_IP: Ipv4Addr = Ipv4Addr::new(192, 168, 7, 212);

const MAX_DATAGRAM_SIZE: usize = 65_507;

// M-SEARCH * HTTP/1.1
//     HOST: 239.255.255.250:1900
//     MAN: "ssdp:discover"
//     MX: seconds to delay response
//     ST: search target
//     USER-AGENT: OS/version UPnP/2.0 product/version
//     CPFN.UPNP.ORG: friendly name of the control point
//     CPUUID.UPNP.ORG: uuid of the control point

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, SSDP_PORT)).await?;
    println!("bound on {}:{}", SSDP_ADDRESS, SSDP_PORT);

    socket.set_multicast_loop_v4(true)?;
    socket.join_multicast_v4(SSDP_ADDRESS, WLAN_IP).unwrap();

    socket.set_multicast_ttl_v4(4)?;
    // socket
    //     .send_to(discovery.as_bytes(), (SSDP_ADDRESS, SSDP_PORT))
    //     .await?;

    let mut data = vec![0u8; MAX_DATAGRAM_SIZE];
    loop {
        let len = socket.recv(&mut data).await?;
        println!("------");
        println!("{}", String::from_utf8_lossy(&data[..len]));
        println!("------");
    }
}
