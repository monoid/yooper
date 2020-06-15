#![cfg(feature = "cli")]

use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, SubCommand,
};
use yooper::Error;

fn validate_secs(v: String) -> Result<(), String> {
    let msg = "Please specify a number between 1 and 5";
    let v: u8 = v.parse().map_err(|_| msg.to_owned())?;

    match v {
        1..=5 => Ok(()),
        _ => Err(msg.to_owned()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = app_from_crate!()
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommands(vec![
            SubCommand::with_name("discover")
                     .about("discover UPnP devices on network")
                     .arg(Arg::with_name("timeout")
                          .short("t")
                          .long("timeout")
                          .takes_value(true)
                          .default_value("5")
                          .value_name("TIMEOUT")
                          .help("How long to wait for devices on the network to respond. 1..5 seconds per the UPnP spec.")
                          .validator(validate_secs)),
            #[cfg(feature = "description")]
            SubCommand::with_name("describe")
                .about("Describe a UPnP device's capabilities")
                .arg(Arg::with_name("url").help("The URL to describe").required(true)),
        ]).get_matches();

    match args.subcommand() {
        ("discover", Some(sub_m)) => discover(sub_m.value_of("timeout").unwrap().parse()?).await,
        #[cfg(feature = "description")]
        ("describe", Some(sub_m)) => describe::run(sub_m.value_of("url").unwrap()).await,
        _ => unreachable!(),
    }
}

async fn discover(secs: u8) -> Result<(), Error> {
    let mut discovery = yooper::discovery::Discovery::new().await?;

    for result in discovery.find(secs).await? {
        println!("{} at {}", result.server, result.location);
        for service in result.services {
            println!("∟ {:?}", service.target)
        }
    }
    Ok(())
}

#[cfg(feature = "description")]
mod describe {
    use yooper::description::{describe, Device};
    use yooper::Error;

    pub async fn run(url: &str) -> Result<(), Error> {
        describe(url).await.and_then(|d| {
            print_device(d, 0);
            Ok(())
        })
    }

    fn print_device(device: Device, indent: u8) {
        let prefix = if indent == 0 {
            "".into()
        } else {
            format!("{}∟ ", "".repeat((indent - 1).into()))
        };
        println!(
            "{}{}{}:{}",
            prefix,
            device
                .device_type
                .vendor_domain
                .map_or("".into(), |v| format!("{}: ", v)),
            device.device_type.device_type,
            device.device_type.version,
        );
        for svc in device.services {
            println!(
                "{}{}{}:{} -> {}",
                prefix,
                svc.service_type
                    .vendor_domain
                    .map_or("".into(), |v| format!("{}: ", v)),
                svc.service_type.service_type,
                svc.service_type.version,
                svc.control_url,
            )
        }

        for dvc in device.devices {
            print_device(dvc, indent + 1)
        }
    }
}
