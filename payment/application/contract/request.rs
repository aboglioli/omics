use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use publishing::domain::publication::{PublicationId, PublicationRepository};

use crate::domain::contract::{Contract, ContractRepository};

pub struct Request<'a> {
    event_pub: &'a dyn EventPublisher,

    contract_repo: &'a dyn ContractRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> Request<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        contract_repo: &'a dyn ContractRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        Request {
            event_pub,
            contract_repo,
            publication_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, publication_id: String) -> Result<CommandResponse> {
        let publication = self
            .publication_repo
            .find_by_id(&PublicationId::new(publication_id)?)
            .await?;

        if publication.author_id().value() != auth_id {
            return Err(Error::not_owner("publication"));
        }

        let mut contract = Contract::new(self.contract_repo.next_id().await?, &publication)?;

        self.contract_repo.save(&mut contract).await?;

        self.event_pub
            .publish_all(contract.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
