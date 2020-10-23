use std::sync::Arc;

use crate::cache::Cache;
use crate::config::Config;
use crate::error::Error;
use crate::result::Result;

pub struct ConfigService {
    cache: Arc<dyn Cache<String, String>>,
}

impl ConfigService {
    pub fn new(cache: Arc<dyn Cache<String, String>>) -> Result<Self> {
        Ok(ConfigService { cache })
    }

    pub async fn get(&self) -> Result<Config> {
        Ok(Config::get())
    }

    pub async fn minimum_donation_amount<S: Into<String>>(&self, k: S) -> Result<f64> {
        if let Some(v) = self.cache.get(&k.into()).await {
            return v
                .parse()
                .map_err(|err| Error::bad_format("minimum_donation_amount").wrap_raw(err));
        }

        return Err(Error::not_found("minimum_donation_amount"));
    }
}
