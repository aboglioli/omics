use serde::Serialize;

use common::config::ConfigService;
use common::error::Error;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};

#[derive(Serialize)]
pub struct ConfigurationDto {
    days_to_generate_summaries: u64,
    donation_percentage_retention: f64,
    minimum_charge_amount: f64,
    minimum_donation_amount: f64,
    minimum_views_percentage_to_require_contract: f64,
    subscription_percentage_retention: f64,
}

pub struct Get<'a> {
    user_repo: &'a dyn UserRepository,

    config_serv: &'a ConfigService,
}

impl<'a> Get<'a> {
    pub fn new(user_repo: &'a dyn UserRepository, config_serv: &'a ConfigService) -> Self {
        Get {
            user_repo,
            config_serv,
        }
    }

    pub async fn exec(&self, auth_id: String) -> Result<ConfigurationDto> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.is_admin() {
            return Err(Error::unauthorized());
        }

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
