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
