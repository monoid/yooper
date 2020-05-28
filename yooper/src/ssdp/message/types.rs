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

impl ToString for ManDiscover{
    fn to_string(&self) -> String {
        String::from("ssdp:discover")
    }
}

impl FromStr for ManDiscover {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ssdp:discover" => Ok(Self {}),
            _ => Err(Error::IncorrectHeader("man")),
        }
    }
}
