mod repository;
mod status;
pub use repository::*;
pub use status::*;

use common::error::Error;
use common::event::Event;
use common::model::{AggregateRoot, StatusHistory, StringId};
use common::result::Result;

use crate::domain::admin::Admin;
use crate::domain::publication::Publication;

pub type ContractId = StringId;

#[derive(Debug, Clone)]
pub struct Contract {
    base: AggregateRoot<ContractId, Event>,
    publication: Publication,
    status_history: StatusHistory<Status>,
}

impl Contract {
    pub fn new(id: ContractId, publication: Publication) -> Result<Self> {
        if publication.statistics().unique_views() < 1000 {
            return Err(Error::new("contract", "publication_has_low_views"));
        }

        Ok(Contract {
            base: AggregateRoot::new(id),
            publication,
            status_history: StatusHistory::new(Status::Requested),
        })
    }

    pub fn base(&self) -> &AggregateRoot<ContractId, Event> {
        &self.base
    }

    pub fn publication(&self) -> &Publication {
        &self.publication
    }

    pub fn status_history(&self) -> &StatusHistory<Status> {
        &self.status_history
    }

    pub fn approve(&mut self, admin: &Admin) -> Result<()> {
        if !matches!(self.status_history().current(), Status::Requested) {
            return Err(Error::new("contract", "not_requested"));
        }

        self.status_history.add_status(Status::Approved {
            admin_id: admin.base().id().clone(),
        });

        Ok(())
    }

    pub fn reject(&mut self, admin: &Admin) -> Result<()> {
        if !matches!(self.status_history().current(), Status::Requested) {
            return Err(Error::new("contract", "not_requested"));
        }

        self.status_history.add_status(Status::Rejected {
            admin_id: admin.base().id().clone(),
        });

        Ok(())
    }

    pub fn cancel(&mut self) -> Result<()> {
        self.status_history.add_status(Status::Cancelled);

        Ok(())
    }
}
