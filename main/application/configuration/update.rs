use common::config::ConfigService;
use common::error::Error;
use common::request::CommandResponse;
use common::result::Result;
use identity::domain::user::{UserRepository};
use identity::UserIdAndRole;

use common::config::BusinessRules;

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

    pub async fn exec(
        &self,
        (_auth_id, auth_role): UserIdAndRole,
        cmd: BusinessRules,
    ) -> Result<CommandResponse> {
        if !auth_role.can("change_business_rules") {
            return Err(Error::unauthorized());
        }

        self.config_serv.save_business_rules(cmd).await?;

        Ok(CommandResponse::default())
    }
}
