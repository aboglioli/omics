mod repository;
mod status;
pub use repository::*;
pub use status::*;

use common::error::Error;

use common::model::{AggregateRoot, Events, StatusHistory, StringId};
use common::result::Result;
use identity::domain::user::User;
use publishing::domain::publication::{Publication, PublicationId};
use shared::event::ContractEvent;

pub type ContractId = StringId;

#[derive(Debug, Clone)]
pub struct Contract {
    base: AggregateRoot<ContractId>,
    events: Events<ContractEvent>,
    publication_id: PublicationId,
    status_history: StatusHistory<Status>,
}

impl Contract {
    pub fn new(id: ContractId, publication: &Publication) -> Result<Self> {
        if publication.statistics().unique_views() < 1000 {
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
        status_history: StatusHistory<Status>,
    ) -> Self {
        Contract {
            base,
            events: Events::new(),
            publication_id,
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
