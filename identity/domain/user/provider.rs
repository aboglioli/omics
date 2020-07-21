use common::error::Error;

#[derive(Debug, Clone)]
pub enum Provider {
    Local,
    Google,
    Facebook,
}

impl Provider {
    pub fn new(provider: &str) -> Result<Provider, Error> {
        match provider.to_lowercase().as_ref() {
            "local" => Ok(Provider::Local),
            "google" => Ok(Provider::Google),
            "facebook" => Ok(Provider::Facebook),
            _ => Err(Error::application()),
        }
    }
}
