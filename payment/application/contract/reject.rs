use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use identity::UserIdAndRole;

use crate::domain::contract::{ContractId, ContractRepository};

pub struct Reject<'a> {
    event_pub: &'a dyn EventPublisher,

    contract_repo: &'a dyn ContractRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Reject<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        contract_repo: &'a dyn ContractRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        Reject {
            event_pub,
            contract_repo,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        contract_id: String,
    ) -> Result<CommandResponse> {
        if !auth_role.can("reject_contract") {
            return Err(Error::unauthorized());
        }

        let user = self.user_repo.find_by_id(&auth_id).await?;

        let mut contract = self
            .contract_repo
            .find_by_id(&ContractId::new(contract_id)?)
            .await?;
        contract.reject(&user)?;

        self.contract_repo.save(&mut contract).await?;

        self.event_pub
            .publish_all(contract.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
