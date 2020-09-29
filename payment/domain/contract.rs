mod repository;
mod status;
pub use repository::*;
pub use status::*;

use common::error::Error;
use common::event::Event;
use common::model::{AggregateRoot, Events, StatusHistory, StringId};
use common::result::Result;
use identity::domain::user::User;
use publishing::domain::publication::{Publication, PublicationId};

pub type ContractId = StringId;

#[derive(Debug, Clone)]
pub struct Contract {
    base: AggregateRoot<ContractId>,
    events: Events<Event>,
    publication_id: PublicationId,
    status_history: StatusHistory<Status>,
}

impl Contract {
    pub fn new(id: ContractId, publication: &Publication) -> Result<Self> {
        if publication.statistics().unique_views() < 1000 {
            return Err(Error::new("contract", "publication_has_low_views"));
        }

        Ok(Contract {
            base: AggregateRoot::new(id),
            events: Events::new(),
            publication_id: publication.base().id().clone(),
            status_history: StatusHistory::new(Status::init()),
        })
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

    pub fn events(&self) -> &Events<Event> {
        &self.events
    }

    pub fn publication_id(&self) -> &PublicationId {
        &self.publication_id
    }

    pub fn status_history(&self) -> &StatusHistory<Status> {
        &self.status_history
    }

    pub fn approve(&mut self, user: &User) -> Result<()> {
        let status = self
            .status_history
            .current()
            .approve(user.base().id().clone())?;
        self.status_history.add_status(status);
        self.base.update();

        self.status_history.add_status(Status::Approved {
            admin_id: user.base().id().clone(),
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

        self.status_history.add_status(Status::Rejected {
            admin_id: user.base().id().clone(),
        });

        Ok(())
    }

    pub fn cancel(&mut self) -> Result<()> {
        let status = self.status_history.current().cancel()?;
        self.status_history.add_status(status);
        self.base.update();

        Ok(())
    }
}
