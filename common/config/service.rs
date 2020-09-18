use crate::config::Config;
use crate::result::Result;

pub struct ConfigService;

impl ConfigService {
    pub fn new() -> Result<Self> {
        Ok(ConfigService)
    }

    pub async fn get(&self) -> Result<Config> {
        Ok(Config::get())
    }
}
