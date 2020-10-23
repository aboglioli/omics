use common::config::ConfigService;
use common::error::Error;
use common::request::CommandResponse;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};

use crate::application::dtos::ConfigurationDto;

pub struct Update<'a> {
    user_repo: &'a dyn UserRepository,

    config_serv: &'a ConfigService,
}

impl<'a> Update<'a> {
    pub fn new(user_repo: &'a dyn UserRepository, config_serv: &'a ConfigService) -> Self {
        Update {
            user_repo,
            config_serv,
        }
    }

    pub async fn exec(&self, auth_id: String, cmd: ConfigurationDto) -> Result<CommandResponse> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.is_admin() {
            return Err(Error::unauthorized());
        }

        self.config_serv
            .set("days_to_generate_summaries", cmd.days_to_generate_summaries)
            .await?;
        self.config_serv
            .set(
                "donation_percentage_retention",
                cmd.donation_percentage_retention,
            )
            .await?;
        self.config_serv
            .set("minimum_charge_amount", cmd.minimum_charge_amount)
            .await?;
        self.config_serv
            .set("minimum_donation_amount", cmd.minimum_donation_amount)
            .await?;
        self.config_serv
            .set(
                "minimum_views_percentage_to_require_contract",
                cmd.minimum_views_percentage_to_require_contract,
            )
            .await?;
        self.config_serv
            .set(
                "subscription_percentage_retention",
                cmd.subscription_percentage_retention,
            )
            .await?;

        Ok(CommandResponse::default())
    }
}
