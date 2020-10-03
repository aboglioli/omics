use std::str::FromStr;

use serde::{Deserialize, Serialize};

use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "income")]
    Income,
    #[serde(rename = "outcome")]
    Outcome,
    #[serde(rename = "transfer")]
    Transfer,
}

impl ToString for Kind {
    fn to_string(&self) -> String {
        match self {
            Kind::Income => "income".to_owned(),
            Kind::Outcome => "outcome".to_owned(),
            Kind::Transfer => "transfer".to_owned(),
        }
    }
}

impl FromStr for Kind {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "income" => Ok(Kind::Income),
            "outcome" => Ok(Kind::Outcome),
            "transfer" => Ok(Kind::Transfer),
            _ => Err(Error::not_found("kind")),
        }
    }
}
