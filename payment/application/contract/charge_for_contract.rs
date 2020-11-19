use common::config::ConfigService;
use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;
use identity::domain::user::UserRepository;
use identity::UserIdAndRole;
use publishing::domain::publication::PublicationRepository;

use crate::domain::contract::{ContractId, ContractRepository};
use crate::domain::payment::PaymentService;

pub struct ChargeForContract<'a> {
    event_pub: &'a dyn EventPublisher,

    contract_repo: &'a dyn ContractRepository,
    publication_repo: &'a dyn PublicationRepository,
    user_repo: &'a dyn UserRepository,

    config_serv: &'a ConfigService,
    payment_serv: &'a dyn PaymentService,
}

impl<'a> ChargeForContract<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        contract_repo: &'a dyn ContractRepository,
        publication_repo: &'a dyn PublicationRepository,
        user_repo: &'a dyn UserRepository,
        config_serv: &'a ConfigService,
        payment_serv: &'a dyn PaymentService,
    ) -> Self {
        ChargeForContract {
            event_pub,
            contract_repo,
            publication_repo,
            user_repo,
            config_serv,
            payment_serv,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        contract_id: String,
    ) -> Result<()> {
        let mut contract = self
            .contract_repo
            .find_by_id(&ContractId::new(contract_id)?)
            .await?;
        let publication = self
            .publication_repo
            .find_by_id(contract.publication_id())
            .await?;

        if publication.author_id() != &auth_id || !auth_role.can("charge_for_contract") {
            return Err(Error::not_owner("contract"));
        }

        let user = self.user_repo.find_by_id(&auth_id).await?;
        if user.payment_email().is_none() {
            return Err(Error::new("contract", "missing_payment_email"));
        }

        let payment = contract.pay_summaries()?;

        let business_rules = self.config_serv.get_business_rules().await?;
        if payment.amount().value() < business_rules.minimum_charge_amount {
            return Err(Error::new("contract", "minimum_charge"));
        }

        self.payment_serv
            .send_payment(
                user.payment_email().unwrap().to_string(),
                format!(
                    "Pago de Omics por contrato de {}",
                    publication.header().name().to_string()
                ),
                payment.amount().value(),
            )
            .await?;

        self.contract_repo.save(&mut contract).await?;

        self.event_pub
            .publish_all(contract.events().to_vec()?)
            .await?;

        Ok(())
    }
}
