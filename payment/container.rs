use std::sync::Arc;

use async_trait::async_trait;

use common::container::Container;
use common::event::EventPublisher;

use identity::domain::user::UserRepository;
use publishing::domain::publication::{PublicationRepository, StatisticsService};
use publishing::domain::reader::ReaderRepository;

use crate::domain::contract::{ContractRepository, ContractService};
use crate::domain::payment::PaymentService;
use crate::domain::plan::PlanRepository;
use crate::domain::subscription::SubscriptionRepository;

pub struct PaymentContainer<EPub> {
    event_pub: Arc<EPub>,

    contract_repo: Arc<dyn ContractRepository>,
    plan_repo: Arc<dyn PlanRepository>,
    publication_repo: Arc<dyn PublicationRepository>,
    reader_repo: Arc<dyn ReaderRepository>,
    subscription_repo: Arc<dyn SubscriptionRepository>,
    user_repo: Arc<dyn UserRepository>,

    contract_serv: Arc<ContractService>,
    payment_serv: Arc<dyn PaymentService>,
}

impl<EPub> PaymentContainer<EPub>
where
    EPub: EventPublisher,
{
    pub fn new(
        event_pub: Arc<EPub>,
        contract_repo: Arc<dyn ContractRepository>,
        plan_repo: Arc<dyn PlanRepository>,
        publication_repo: Arc<dyn PublicationRepository>,
        reader_repo: Arc<dyn ReaderRepository>,
        subscription_repo: Arc<dyn SubscriptionRepository>,
        user_repo: Arc<dyn UserRepository>,
        payment_serv: Arc<dyn PaymentService>,
        statistics_serv: Arc<StatisticsService>,
    ) -> Self {
        let contract_serv = Arc::new(ContractService::new(
            contract_repo.clone(),
            subscription_repo.clone(),
            statistics_serv.clone(),
        ));

        PaymentContainer {
            event_pub,
            contract_repo,
            plan_repo,
            publication_repo,
            reader_repo,
            subscription_repo,
            user_repo,
            contract_serv,
            payment_serv,
        }
    }

    pub fn event_pub(&self) -> &EPub {
        &self.event_pub
    }

    pub fn contract_repo(&self) -> &dyn ContractRepository {
        self.contract_repo.as_ref()
    }

    pub fn plan_repo(&self) -> &dyn PlanRepository {
        self.plan_repo.as_ref()
    }

    pub fn publication_repo(&self) -> &dyn PublicationRepository {
        self.publication_repo.as_ref()
    }

    pub fn reader_repo(&self) -> &dyn ReaderRepository {
        self.reader_repo.as_ref()
    }

    pub fn subscription_repo(&self) -> &dyn SubscriptionRepository {
        self.subscription_repo.as_ref()
    }

    pub fn user_repo(&self) -> &dyn UserRepository {
        self.user_repo.as_ref()
    }

    pub fn contract_serv(&self) -> &ContractService {
        &self.contract_serv
    }

    pub fn payment_serv(&self) -> &dyn PaymentService {
        self.payment_serv.as_ref()
    }
}

#[async_trait]
impl<EPub> Container for PaymentContainer<EPub> where EPub: Sync + Send {}
