use common::config::ConfigService;
use common::error::Error;
use common::request::CommandResponse;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};

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

    pub async fn exec(&self, auth_id: String, cmd: BusinessRules) -> Result<CommandResponse> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.is_admin() {
            return Err(Error::unauthorized());
        }

        self.config_serv.save_business_rules(cmd).await?;

        Ok(CommandResponse::default())
    }
}
