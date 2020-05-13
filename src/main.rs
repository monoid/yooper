use std::error::Error;
use std::net::Ipv4Addr;
use tokio::net::UdpSocket;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const OS: &'static str = "linux"; //TODO
const SSDP_ADDRESS: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
const SSDP_PORT: u16 = 1900;

const WLAN_IP: Ipv4Addr = Ipv4Addr::new(192, 168, 7, 212);

const MAX_DATAGRAM_SIZE: usize = 65_507;

const UUID: &str = "6262ef8c-c0e2-4a6a-b4ab-37d07792f996";

fn make_request(req: Vec<(&str, String)>) -> String {
    req.into_iter()
        .map(|(k, v)| format!("{}: {}\r\n", k, v))
        .collect::<Vec<String>>()
        .join("")
}

// M-SEARCH * HTTP/1.1
//     HOST: 239.255.255.250:1900
//     MAN: "ssdp:discover"
//     MX: seconds to delay response
//     ST: search target
//     USER-AGENT: OS/version UPnP/2.0 product/version
//     CPFN.UPNP.ORG: friendly name of the control point
//     CPUUID.UPNP.ORG: uuid of the control point

fn get_discovery_msg(secs: u32) -> String {
    let headers = make_request(vec![
        ("host", format!("{}:{}", SSDP_ADDRESS, SSDP_PORT)),
        ("man", "\"ssdp:discover\"".into()),
        ("mx", secs.to_string()),
        ("st", "ssdp::all".into()),
        (
            "user-agent".into(),
            format!("{}/5.1 UPnP/2.0 yooper/{}", OS, VERSION),
        ),
        ("cpfn.upnp.org", "yooper".into()),
        ("cpuuid.upnp.org", UUID.into()),
    ]);
    format!("M-SEARCH * HTTP/1.1\r\n{}\r\n", headers)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, SSDP_PORT)).await?;
    println!("bound on {}:{}", SSDP_ADDRESS, SSDP_PORT);

    socket.set_multicast_loop_v4(true)?;
    socket.join_multicast_v4(SSDP_ADDRESS, WLAN_IP).unwrap();

    let discovery = get_discovery_msg(120);
    socket.set_multicast_ttl_v4(4)?;
    socket
        .send_to(discovery.as_bytes(), (SSDP_ADDRESS, SSDP_PORT))
        .await?;

    let mut data = vec![0u8; MAX_DATAGRAM_SIZE];
    loop {
        let len = socket.recv(&mut data).await?;
        println!("------");
        println!("{}", String::from_utf8_lossy(&data[..len]));
        println!("------");
    }
}
