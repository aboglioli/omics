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

    pub async fn days_to_generate_summaries(&self) -> Result<u64> {
        self.get_from_cache("days_to_generate_summaries").await
    }

    pub async fn donation_percentage_retention(&self) -> Result<f64> {
        self.get_from_cache("donation_percentage_retention").await
    }

    pub async fn minimum_charge_amount(&self) -> Result<f64> {
        self.get_from_cache("minimum_charge_amount").await
    }

    pub async fn minimum_donation_amount(&self) -> Result<f64> {
        self.get_from_cache("minimum_donation_amount").await
    }

    pub async fn minimum_views_percentage_to_require_contract(&self) -> Result<f64> {
        self.get_from_cache("minimum_views_percentage_to_require_contract")
            .await
    }

    pub async fn subscription_percentage_retention(&self) -> Result<f64> {
        self.get_from_cache("subscription_percentage_retention")
            .await
    }
}
