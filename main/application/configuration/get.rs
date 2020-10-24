use common::config::{BusinessRules, ConfigService};
use common::result::Result;

pub struct Get<'a> {
    config_serv: &'a ConfigService,
}

impl<'a> Get<'a> {
    pub fn new(config_serv: &'a ConfigService) -> Self {
        Get { config_serv }
    }

    pub async fn exec(&self) -> Result<BusinessRules> {
        self.config_serv.get_business_rules().await
    }
}
