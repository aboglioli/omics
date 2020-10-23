use common::config::ConfigService;
use common::result::Result;

use crate::application::dtos::ConfigurationDto;

pub struct Get<'a> {
    config_serv: &'a ConfigService,
}

impl<'a> Get<'a> {
    pub fn new(config_serv: &'a ConfigService) -> Self {
        Get { config_serv }
    }

    pub async fn exec(&self) -> Result<ConfigurationDto> {
        Ok(ConfigurationDto {
            days_to_generate_summaries: self.config_serv.get("days_to_generate_summaries").await?,
            donation_percentage_retention: self
                .config_serv
                .get("donation_percentage_retention")
                .await?,
            minimum_charge_amount: self.config_serv.get("minimum_charge_amount").await?,
            minimum_donation_amount: self.config_serv.get("minimum_donation_amount").await?,
            minimum_views_percentage_to_require_contract: self
                .config_serv
                .get("minimum_views_percentage_to_require_contract")
                .await?,
            subscription_percentage_retention: self
                .config_serv
                .get("subscription_percentage_retention")
                .await?,
        })
    }
}
