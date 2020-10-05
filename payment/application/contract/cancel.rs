use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use publishing::domain::publication::PublicationRepository;

use crate::domain::contract::{ContractId, ContractRepository};

pub struct Cancel<'a> {
    event_pub: &'a dyn EventPublisher,

    contract_repo: &'a dyn ContractRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> Cancel<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        contract_repo: &'a dyn ContractRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        Cancel {
            event_pub,
            contract_repo,
            publication_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, contract_id: String) -> Result<CommandResponse> {
        let mut contract = self
            .contract_repo
            .find_by_id(&ContractId::new(contract_id)?)
            .await?;

        let publication = self
            .publication_repo
            .find_by_id(contract.publication_id())
            .await?;

        if publication.author_id().value() != auth_id {
            return Err(Error::not_owner("publication"));
        }

        contract.cancel()?;

        self.contract_repo.save(&mut contract).await?;

        self.event_pub
            .publish_all(contract.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
