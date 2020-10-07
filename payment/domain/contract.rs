mod repository;
mod service;
mod status;
mod summary;
pub use repository::*;
pub use service::*;
pub use status::*;
pub use summary::*;

use common::error::Error;
use common::model::{AggregateRoot, Events, StatusHistory, StringId};
use common::result::Result;
use identity::domain::user::User;
use publishing::domain::publication::{Publication, PublicationId};
use shared::event::ContractEvent;

use crate::domain::payment::{Amount, Kind, Payment};

pub type ContractId = StringId;

#[derive(Debug, Clone)]
pub struct Contract {
    base: AggregateRoot<ContractId>,
    events: Events<ContractEvent>,
    publication_id: PublicationId,
    summaries: Vec<Summary>,
    payments: Vec<Payment>,
    status_history: StatusHistory<Status>,
}

impl Contract {
    pub fn new(id: ContractId, publication: &Publication) -> Result<Self> {
        if publication.statistics().unique_views() < 30 {
            return Err(Error::new("contract", "publication_has_low_views"));
        }

        if !publication.is_published() {
            return Err(Error::new("contract", "publication_is_not_published"));
        }

        if publication.has_contract() {
            return Err(Error::new("contract", "publication_already_has_contract"));
        }

        let mut contract = Contract {
            base: AggregateRoot::new(id),
            events: Events::new(),
            publication_id: publication.base().id().clone(),
            summaries: Vec::new(),
            payments: Vec::new(),
            status_history: StatusHistory::new(Status::init()),
        };

        contract.events.record_event(ContractEvent::Requested {
            id: contract.base().id().to_string(),
            publication_id: contract.publication_id().to_string(),
            author_id: publication.author_id().to_string(),
        });

        Ok(contract)
    }

    pub fn build(
        base: AggregateRoot<ContractId>,
        publication_id: PublicationId,
        summaries: Vec<Summary>,
        payments: Vec<Payment>,
        status_history: StatusHistory<Status>,
    ) -> Self {
        Contract {
            base,
            events: Events::new(),
            publication_id,
            summaries,
            payments,
            status_history,
        }
    }

    pub fn base(&self) -> &AggregateRoot<ContractId> {
        &self.base
    }

    pub fn events(&self) -> &Events<ContractEvent> {
        &self.events
    }

    pub fn publication_id(&self) -> &PublicationId {
        &self.publication_id
    }

    pub fn summaries(&self) -> &[Summary] {
        &self.summaries
    }

    pub fn payments(&self) -> &[Payment] {
        &self.payments
    }

    pub fn status_history(&self) -> &StatusHistory<Status> {
        &self.status_history
    }

    pub fn is_active(&self) -> bool {
        self.base().deleted_at().is_none()
            && matches!(self.status_history().current(), Status::Approved{ .. })
    }

    pub fn approve(&mut self, user: &User) -> Result<()> {
        let status = self
            .status_history
            .current()
            .approve(user.base().id().clone())?;
        self.status_history.add_status(status);
        self.base.update();

        self.events.record_event(ContractEvent::Approved {
            id: self.base().id().to_string(),
            publication_id: self.publication_id().to_string(),
            admin_id: user.base().id().to_string(),
        });

        Ok(())
    }

    pub fn reject(&mut self, user: &User) -> Result<()> {
        let status = self
            .status_history
            .current()
            .reject(user.base().id().clone())?;
        self.status_history.add_status(status);
        self.base.update();

        self.events.record_event(ContractEvent::Rejected {
            id: self.base().id().to_string(),
            publication_id: self.publication_id().to_string(),
            admin_id: user.base().id().to_string(),
        });

        Ok(())
    }

    pub fn cancel(&mut self) -> Result<()> {
        let status = self.status_history.current().cancel()?;
        self.status_history.add_status(status);
        self.base.update();

        self.events.record_event(ContractEvent::Cancelled {
            id: self.base().id().to_string(),
            publication_id: self.publication_id().to_string(),
        });

        Ok(())
    }

    pub fn add_summary(&mut self, summary: Summary) -> Result<()> {
        if !self.is_active() {
            return Err(Error::new("contract", "not_active"));
        }

        if let Some(last_summary) = self.summaries().last() {
            if summary.from() < last_summary.to() {
                return Err(Error::new("summary", "date_should_be_the_last"));
            }
        }

        self.summaries.push(summary.clone());

        self.events.record_event(ContractEvent::SummaryAdded {
            id: self.base().id().to_string(),
            publication_id: self.publication_id().to_string(),
            total: summary.total(),
            amount: summary.amount(),
            from: summary.from().to_rfc3339(),
            to: summary.to().to_rfc3339(),
        });

        Ok(())
    }

    pub fn pay_summaries(&mut self) -> Result<Payment> {
        let mut amount = 0.0;

        for summary in self.summaries.iter_mut() {
            if summary.is_paid() {
                continue;
            }

            amount += summary.amount();
            summary.pay()?;
        }

        let payment = Payment::new(Kind::Outcome, Amount::new(amount)?)?;
        self.payments.push(payment.clone());

        self.events.record_event(ContractEvent::PaymentAdded {
            id: self.base().id().to_string(),
            publication_id: self.publication_id().to_string(),
            amount,
        });

        Ok(payment)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    use chrono::DateTime;

    use identity::mocks as identity_mocks;
    use publishing::domain::publication::Statistics;
    use publishing::mocks as publishing_mocks;

    #[test]
    fn low_statistics() {
        let mut publication = publishing_mocks::publication(
            "#publication01",
            "#user01",
            "Publication 1",
            "#category01",
            vec!["Tags"],
            "domain.com/cover.jpg",
            3,
            true,
            true,
            false,
        );
        assert!(Contract::new(ContractId::new("#contract01").unwrap(), &publication,).is_err());

        publication
            .set_statistics(Statistics::new(500, 100, 45, 26, 2, 3.2).unwrap())
            .unwrap();
        assert!(Contract::new(ContractId::new("#contract01").unwrap(), &publication,).is_err());
    }

    #[test]
    fn already_has_a_contract() {
        let mut publication = publishing_mocks::publication(
            "#publication01",
            "#user01",
            "Publication 1",
            "#category01",
            vec!["Tags"],
            "domain.com/cover.jpg",
            3,
            true,
            true,
            true,
        );
        publication
            .set_statistics(Statistics::new(10000, 10000, 10000, 10000, 10000, 4.5).unwrap())
            .unwrap();
        assert!(Contract::new(ContractId::new("#contract01").unwrap(), &publication,).is_err());
    }

    #[test]
    fn valid() {
        let mut publication = publishing_mocks::publication(
            "#publication01",
            "#user01",
            "Publication 1",
            "#category01",
            vec!["Tags"],
            "domain.com/cover.jpg",
            3,
            true,
            true,
            false,
        );
        publication
            .set_statistics(Statistics::new(10000, 10000, 10000, 10000, 10000, 4.5).unwrap())
            .unwrap();
        let contract =
            Contract::new(ContractId::new("#contract01").unwrap(), &publication).unwrap();

        assert_eq!(contract.status_history().current().to_string(), "requested");

        assert!(!contract.events().to_vec().unwrap().is_empty());
    }

    #[test]
    fn approve() {
        let mut publication = publishing_mocks::publication(
            "#publication01",
            "#user01",
            "Publication 1",
            "#category01",
            vec!["Tags"],
            "domain.com/cover.jpg",
            3,
            true,
            true,
            false,
        );
        publication
            .set_statistics(Statistics::new(10000, 10000, 10000, 10000, 10000, 4.5).unwrap())
            .unwrap();
        let mut contract =
            Contract::new(ContractId::new("#contract01").unwrap(), &publication).unwrap();

        let admin = identity_mocks::user(
            "#admin01",
            "admin-1",
            "admin@omics.com",
            "P@asswd!",
            true,
            None,
            None,
            "admin",
        );

        contract.approve(&admin).unwrap();

        assert_eq!(contract.status_history().current().to_string(), "approved");
        assert_eq!(contract.status_history().history().len(), 2);
        assert!(contract.is_active());

        assert!(!contract.events().to_vec().unwrap().is_empty());
    }

    #[test]
    fn reject() {
        let mut publication = publishing_mocks::publication(
            "#publication01",
            "#user01",
            "Publication 1",
            "#category01",
            vec!["Tags"],
            "domain.com/cover.jpg",
            3,
            true,
            true,
            false,
        );
        publication
            .set_statistics(Statistics::new(10000, 10000, 10000, 10000, 10000, 4.5).unwrap())
            .unwrap();
        let mut contract =
            Contract::new(ContractId::new("#contract01").unwrap(), &publication).unwrap();

        let admin = identity_mocks::user(
            "#admin01",
            "admin-1",
            "admin@omics.com",
            "P@asswd!",
            true,
            None,
            None,
            "admin",
        );

        contract.reject(&admin).unwrap();

        assert_eq!(contract.status_history().current().to_string(), "rejected");
        assert_eq!(contract.status_history().history().len(), 2);
        assert!(!contract.is_active());

        assert!(!contract.events().to_vec().unwrap().is_empty());
    }

    #[test]
    fn cancel() {
        let mut publication = publishing_mocks::publication(
            "#publication01",
            "#user01",
            "Publication 1",
            "#category01",
            vec!["Tags"],
            "domain.com/cover.jpg",
            3,
            true,
            true,
            false,
        );
        publication
            .set_statistics(Statistics::new(10000, 10000, 10000, 10000, 10000, 4.5).unwrap())
            .unwrap();
        let mut contract =
            Contract::new(ContractId::new("#contract01").unwrap(), &publication).unwrap();

        let admin = identity_mocks::user(
            "#admin01",
            "admin-1",
            "admin@omics.com",
            "P@asswd!",
            true,
            None,
            None,
            "admin",
        );

        contract.approve(&admin).unwrap();
        contract.cancel().unwrap();

        assert_eq!(contract.status_history().current().to_string(), "cancelled");
        assert_eq!(contract.status_history().history().len(), 3);
        assert!(!contract.is_active());

        assert!(!contract.events().to_vec().unwrap().is_empty());
    }

    #[test]
    fn add_summaries() {
        let mut publication = publishing_mocks::publication(
            "#publication01",
            "#user01",
            "Publication 1",
            "#category01",
            vec!["Tags"],
            "domain.com/cover.jpg",
            3,
            true,
            true,
            false,
        );
        publication
            .set_statistics(Statistics::new(10000, 10000, 10000, 10000, 10000, 4.5).unwrap())
            .unwrap();
        let mut contract =
            Contract::new(ContractId::new("#contract01").unwrap(), &publication).unwrap();
        let admin = identity_mocks::user(
            "#admin01",
            "admin-1",
            "admin@omics.com",
            "P@asswd!",
            true,
            None,
            None,
            "admin",
        );

        let summary_statistics = Statistics::new(4000, 4000, 4000, 4000, 4000, 3.8).unwrap();

        assert!(contract
            .add_summary(
                Summary::new(
                    summary_statistics.clone(),
                    4000.0,
                    2000.0,
                    DateTime::from_str("2020-05-01T14:30:00Z").unwrap(),
                    DateTime::from_str("2020-05-30T14:30:00Z").unwrap(),
                )
                .unwrap()
            )
            .is_err());

        contract.approve(&admin).unwrap();

        assert!(contract
            .add_summary(
                Summary::new(
                    summary_statistics.clone(),
                    4000.0,
                    2000.0,
                    DateTime::from_str("2020-05-01T14:30:00Z").unwrap(),
                    DateTime::from_str("2020-05-30T14:30:00Z").unwrap(),
                )
                .unwrap()
            )
            .is_ok());
        assert!(contract
            .add_summary(
                Summary::new(
                    summary_statistics.clone(),
                    4000.0,
                    2000.0,
                    DateTime::from_str("2020-05-05T14:30:00Z").unwrap(),
                    DateTime::from_str("2020-05-28T14:30:00Z").unwrap(),
                )
                .unwrap()
            )
            .is_err());
        assert!(contract
            .add_summary(
                Summary::new(
                    summary_statistics.clone(),
                    4000.0,
                    2000.0,
                    DateTime::from_str("2020-05-20T14:30:00Z").unwrap(),
                    DateTime::from_str("2020-06-05T14:30:00Z").unwrap(),
                )
                .unwrap()
            )
            .is_err());
        assert!(contract
            .add_summary(
                Summary::new(
                    summary_statistics.clone(),
                    4000.0,
                    2000.0,
                    DateTime::from_str("2020-05-31T14:30:00Z").unwrap(),
                    DateTime::from_str("2020-06-20T14:30:00Z").unwrap(),
                )
                .unwrap()
            )
            .is_ok());
    }

    #[test]
    fn pay_unpaid_summaries() {
        let mut publication = publishing_mocks::publication(
            "#publication01",
            "#user01",
            "Publication 1",
            "#category01",
            vec!["Tags"],
            "domain.com/cover.jpg",
            3,
            true,
            true,
            false,
        );
        publication
            .set_statistics(Statistics::new(10000, 10000, 10000, 10000, 10000, 4.5).unwrap())
            .unwrap();
        let mut contract =
            Contract::new(ContractId::new("#contract01").unwrap(), &publication).unwrap();
        let admin = identity_mocks::user(
            "#admin01",
            "admin-1",
            "admin@omics.com",
            "P@asswd!",
            true,
            None,
            None,
            "admin",
        );
        let summary_statistics = Statistics::new(4000, 4000, 4000, 4000, 4000, 3.8).unwrap();

        contract.approve(&admin).unwrap();

        contract
            .add_summary(
                Summary::new(
                    summary_statistics.clone(),
                    4000.0,
                    1000.0,
                    DateTime::from_str("2020-04-01T14:30:00Z").unwrap(),
                    DateTime::from_str("2020-04-30T14:30:00Z").unwrap(),
                )
                .unwrap(),
            )
            .unwrap();
        let payment = contract.pay_summaries().unwrap();
        assert_eq!(payment.amount().value(), 1000.0);
        assert_eq!(contract.payments().len(), 1);

        contract
            .add_summary(
                Summary::new(
                    summary_statistics.clone(),
                    9000.0,
                    2500.0,
                    DateTime::from_str("2020-05-01T14:30:00Z").unwrap(),
                    DateTime::from_str("2020-05-30T14:30:00Z").unwrap(),
                )
                .unwrap(),
            )
            .unwrap();
        contract
            .add_summary(
                Summary::new(
                    summary_statistics.clone(),
                    5000.0,
                    500.0,
                    DateTime::from_str("2020-06-01T14:30:00Z").unwrap(),
                    DateTime::from_str("2020-06-30T14:30:00Z").unwrap(),
                )
                .unwrap(),
            )
            .unwrap();
        let payment = contract.pay_summaries().unwrap();
        assert_eq!(payment.amount().value(), 2500.0 + 500.0);
        assert_eq!(contract.payments().len(), 2);

        contract
            .add_summary(
                Summary::new(
                    summary_statistics.clone(),
                    4000.0,
                    980.75,
                    DateTime::from_str("2020-07-01T14:30:00Z").unwrap(),
                    DateTime::from_str("2020-08-01T14:30:00Z").unwrap(),
                )
                .unwrap(),
            )
            .unwrap();
        let payment = contract.pay_summaries().unwrap();
        assert_eq!(payment.amount().value(), 980.75);
        assert_eq!(contract.payments().len(), 3);

        assert_eq!(contract.summaries().len(), 4);
    }
}
