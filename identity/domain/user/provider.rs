use std::str::FromStr;

use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub enum Provider {
    Local,
    Google,
    Facebook,
}

impl FromStr for Provider {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "local" => Ok(Provider::Local),
            "google" => Ok(Provider::Google),
            "facebook" => Ok(Provider::Facebook),
            _ => Err(Error::not_found("provider")),
        }
    }
}

impl ToString for Provider {
    fn to_string(&self) -> String {
        match self {
            Provider::Local => "local".to_owned(),
            Provider::Google => "google".to_owned(),
            Provider::Facebook => "facebook".to_owned(),
        }
    }
}
