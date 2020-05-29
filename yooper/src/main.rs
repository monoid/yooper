use yooper::{
    Error,
    discovery::Discovery
};

// const VERSION: &'static str = env!("CARGO_PKG_VERSION");
// const OS: &'static str = "linux"; //TODO

// M-SEARCH * HTTP/1.1
//     HOST: 239.255.255.250:1900
//     MAN: "ssdp:discover"
//     MX: seconds to delay response
//     ST: search target
//     USER-AGENT: OS/version UPnP/2.0 product/version
//     CPFN.UPNP.ORG: friendly name of the control point
//     CPUUID.UPNP.ORG: uuid of the control point

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut discovery = Discovery::new().await?;

    for result in discovery.find(5).await? {
        println!("{} at {}", result.server, result.address);
        for service in result.services {
            println!("∟ {}", service.target)
        }

    }
    Ok(())
}
