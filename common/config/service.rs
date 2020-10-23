use std::str::FromStr;
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

    async fn get_from_cache<T>(&self, k: &str) -> Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error,
    {
        if let Some(v) = self.cache.get(&k.into()).await {
            return v.parse().map_err(|err| Error::bad_format(k).wrap_raw(err));
        }

        return Err(Error::not_found(k));
    }

    pub async fn minimum_donation_amount(&self) -> Result<f64> {
        // if let Some(v) = self.cache.get(&"minimum_donation_amount".to_owned()).await {
        //     return v
        //         .parse()
        //         .map_err(|err| Error::bad_format("minimum_donation_amount").wrap_raw(err));
        // }
        //
        // return Err(Error::not_found("minimum_donation_amount"));
        self.get_from_cache("minimum_donation_amount").await
    }
}
