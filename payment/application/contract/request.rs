use serde::Serialize;

use common::event::EventPublisher;
use common::result::Result;

use crate::domain::contract::{Contract, ContractRepository};
use crate::domain::publication::{PublicationId, PublicationRepository};

#[derive(Serialize)]
pub struct RequestReponse {
    id: String,
}

pub struct Request<'a, EPub, CRepo, PRepo> {
    event_pub: &'a EPub,

    contract_repo: &'a CRepo,
    publication_repo: &'a PRepo,
}

impl<'a, EPub, CRepo, PRepo> Request<'a, EPub, CRepo, PRepo>
where
    EPub: EventPublisher,
    CRepo: ContractRepository,
    PRepo: PublicationRepository,
{
    pub fn new(event_pub: &'a EPub, contract_repo: &'a CRepo, publication_repo: &'a PRepo) -> Self {
        Request {
            event_pub,
            contract_repo,
            publication_repo,
        }
    }

    pub async fn exec(&self, publication_id: String) -> Result<RequestReponse> {
        let publication_id = PublicationId::new(publication_id)?;
        let publication = self.publication_repo.find_by_id(&publication_id).await?;

        let mut contract = Contract::new(self.contract_repo.next_id().await?, publication)?;

        self.contract_repo.save(&mut contract).await?;

        self.event_pub
            .publish_all(contract.events().to_vec()?)
            .await?;

        Ok(RequestReponse {
            id: contract.base().id().to_string(),
        })
    }
}
