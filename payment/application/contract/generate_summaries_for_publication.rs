use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;
use publishing::domain::publication::{PublicationId, PublicationRepository};

use crate::application::dtos::ContractDto;
use crate::domain::contract::{ContractRepository, ContractService};

pub struct GenerateSummariesForPublication<'a> {
    event_pub: &'a dyn EventPublisher,

    contract_repo: &'a dyn ContractRepository,
    publication_repo: &'a dyn PublicationRepository,

    contract_serv: &'a ContractService,
}

impl<'a> GenerateSummariesForPublication<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        contract_repo: &'a dyn ContractRepository,
        publication_repo: &'a dyn PublicationRepository,
        contract_serv: &'a ContractService,
    ) -> Self {
        GenerateSummariesForPublication {
            event_pub,
            contract_repo,
            publication_repo,
            contract_serv,
        }
    }

    pub async fn exec(&self, auth_id: String, publication_id: String) -> Result<ContractDto> {
        let publication_id = PublicationId::new(publication_id)?;
        let publication = self.publication_repo.find_by_id(&publication_id).await?;
        if publication.author_id().value() != auth_id {
            return Err(Error::not_owner("publication"));
        }

        let mut contract = self
            .contract_serv
            .calculate_summaries_for_publication(publication.base().id())
            .await?;

        self.contract_repo.save(&mut contract).await?;
        self.event_pub
            .publish_all(contract.events().to_vec()?)
            .await?;

        Ok(ContractDto::from(&contract))
    }
}
