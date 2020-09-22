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
    pub fn new<S: Into<String>>(to: S, title: S, body: S) -> Result<Self> {
        Ok(Email {
            to: to.into(),
            title: title.into(),
            body: body.into(),
        })
    }

    pub fn to(&self) -> &str {
        &self.to
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}
