use serde::Serialize;

use common::event::EventPublisher;
use common::result::Result;

use crate::domain::contract::{Contract, ContractRepository};
use crate::domain::publication::PublicationId;

#[derive(Serialize)]
pub struct RequestReponse {
    id: String,
}

pub struct Request<'a, EPub, CRepo> {
    event_pub: &'a EPub,

    contract_repo: &'a CRepo,
}

impl<'a, EPub, CRepo> Request<'a, EPub, CRepo>
where
    EPub: EventPublisher,
    CRepo: ContractRepository,
{
    pub fn new(event_pub: &'a EPub, contract_repo: &'a CRepo) -> Self {
        Request {
            event_pub,
            contract_repo,
        }
    }

    pub async fn exec(&self, publication_id: String) -> Result<RequestReponse> {
        let mut contract = Contract::new(
            self.contract_repo.next_id().await?,
            PublicationId::new(publication_id)?,
        )?;

        self.contract_repo.save(&mut contract).await?;

        self.event_pub
            .publish_all(contract.base().events()?)
            .await?;

        Ok(RequestReponse {
            id: contract.base().id().value().to_owned(),
        })
    }
}
