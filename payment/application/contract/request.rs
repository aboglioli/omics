use serde::Serialize;

use common::error::Error;
use common::event::EventPublisher;
use identity::UserIdAndRole;

use common::result::Result;
use publishing::domain::publication::{PublicationId, PublicationRepository};

use crate::domain::contract::{Contract, ContractRepository, ContractService};

#[derive(Serialize)]
pub struct RequestResponse {
    id: String,
}

pub struct Request<'a> {
    event_pub: &'a dyn EventPublisher,

    contract_repo: &'a dyn ContractRepository,
    publication_repo: &'a dyn PublicationRepository,

    contract_serv: &'a ContractService,
}

impl<'a> Request<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        contract_repo: &'a dyn ContractRepository,
        publication_repo: &'a dyn PublicationRepository,
        contract_serv: &'a ContractService,
    ) -> Self {
        Request {
            event_pub,
            contract_repo,
            publication_repo,
            contract_serv,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        publication_id: String,
    ) -> Result<RequestResponse> {
        let publication_id = PublicationId::new(publication_id)?;
        let publication = self.publication_repo.find_by_id(&publication_id).await?;

        if publication.author_id() != &auth_id || !auth_role.can("request_contract") {
            return Err(Error::not_owner("publication"));
        }

        self.contract_serv.can_request(&publication_id).await?;

        // TODO: should be done by a domain service
        if let Ok(last) = self
            .contract_repo
            .find_by_publication_id(publication.base().id())
            .await
        {
            self.contract_repo.delete(last.base().id()).await?;
        }

        let mut contract = Contract::new(self.contract_repo.next_id().await?, &publication)?;

        self.contract_repo.save(&mut contract).await?;

        self.event_pub
            .publish_all(contract.events().to_vec()?)
            .await?;

        Ok(RequestResponse {
            id: contract.base().id().to_string(),
        })
    }
}
