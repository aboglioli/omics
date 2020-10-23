use std::str::FromStr;
use std::sync::Arc;

use crate::cache::Cache;

use crate::error::Error;
use crate::result::Result;

pub struct ConfigService {
    cache: Arc<dyn Cache<String, String>>,
}

impl ConfigService {
    pub fn new(cache: Arc<dyn Cache<String, String>>) -> Self {
        ConfigService { cache }
    }

    pub async fn get<K, V>(&self, k: K) -> Result<V>
    where
        V: FromStr,
        <V as FromStr>::Err: std::error::Error,
        K: Into<String>,
    {
        let k: String = k.into();

        if let Some(v) = self.cache.get(&k).await {
            return v.parse().map_err(|err| Error::bad_format(k).wrap_raw(err));
        }

        return Err(Error::not_found(k));
    }

    pub async fn set<K, V>(&self, k: K, v: V) -> Result<()>
    where
        K: Into<String>,
        V: ToString,
    {
        self.cache.set(k.into(), v.to_string()).await?;
        Ok(())
    }

    // TODO: delete
    pub async fn days_to_generate_summaries(&self) -> Result<u64> {
        self.get("days_to_generate_summaries").await
    }

    pub async fn donation_percentage_retention(&self) -> Result<f64> {
        self.get("donation_percentage_retention").await
    }

    pub async fn minimum_charge_amount(&self) -> Result<f64> {
        self.get("minimum_charge_amount").await
    }

    pub async fn minimum_donation_amount(&self) -> Result<f64> {
        self.get("minimum_donation_amount").await
    }

    pub async fn minimum_views_percentage_to_require_contract(&self) -> Result<f64> {
        self.get("minimum_views_percentage_to_require_contract")
            .await
    }

    pub async fn subscription_percentage_retention(&self) -> Result<f64> {
        self.get("subscription_percentage_retention").await
    }
}
