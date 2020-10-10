use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;
use publishing::domain::publication::PublicationRepository;

use crate::domain::contract::{ContractId, ContractRepository};
use crate::domain::payment::PaymentService;

pub struct ChargeForContract<'a> {
    event_pub: &'a dyn EventPublisher,

    contract_repo: &'a dyn ContractRepository,
    publication_repo: &'a dyn PublicationRepository,

    payment_serv: &'a dyn PaymentService,
}

impl<'a> ChargeForContract<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        contract_repo: &'a dyn ContractRepository,
        publication_repo: &'a dyn PublicationRepository,
        payment_serv: &'a dyn PaymentService,
    ) -> Self {
        ChargeForContract {
            event_pub,
            contract_repo,
            publication_repo,
            payment_serv,
        }
    }

    pub async fn exec(&self, auth_id: String, contract_id: String) -> Result<()> {
        let mut contract = self
            .contract_repo
            .find_by_id(&ContractId::new(contract_id)?)
            .await?;
        let publication = self
            .publication_repo
            .find_by_id(contract.publication_id())
            .await?;

        if publication.author_id().value() != auth_id {
            return Err(Error::not_owner("contract"));
        }

        let _payment = contract.pay_summaries()?;

        // TODO: pay to author

        self.contract_repo.save(&mut contract).await?;

        self.event_pub
            .publish_all(contract.events().to_vec()?)
            .await?;

        Ok(())
    }
}
