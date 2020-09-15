mod service;
pub use service::*;

use common::result::Result;

#[derive(Debug, Clone)]
pub struct Email {
    to: String,
    title: String,
    body: String,
}

impl Email {
    pub fn new(to: String, title: String, body: String) -> Result<Self> {
        Ok(Email { to, title, body })
    }
}
