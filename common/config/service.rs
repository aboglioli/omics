use std::sync::Arc;

use crate::cache::Cache;

use crate::config::BusinessRules;
use crate::error::Error;
use crate::result::Result;

pub struct ConfigService {
    cache: Arc<dyn Cache<String, BusinessRules>>,
}

impl ConfigService {
    pub fn new(cache: Arc<dyn Cache<String, BusinessRules>>) -> Self {
        ConfigService { cache }
    }

    pub async fn get_business_rules(&self) -> Result<BusinessRules> {
        if let Some(business_rules) = self.cache.get(&"business_rules".to_owned()).await {
            return Ok(business_rules);
        }

        Err(Error::not_found("business_rules"))
    }

    pub async fn save_business_rules(&self, business_rules: BusinessRules) -> Result<()> {
        self.cache
            .set("business_rules".to_owned(), business_rules)
            .await
    }
}
