use common::error::Error;
use common::event::Event;
use common::model::{AggregateRoot, StatusHistory};
use common::result::Result;

use crate::domain::contract::ContractStatus;
use crate::domain::publication::PublicationId;
use crate::domain::summary::Summary;

pub type ContractId = String;

pub struct Contract {
    base: AggregateRoot<ContractId, Event>,
    publication_id: PublicationId,
    status_history: StatusHistory<ContractStatus>,
    summaries: Vec<Summary>,
}

impl Contract {
    pub fn new(id: ContractId, publication_id: PublicationId) -> Result<Contract> {
        Ok(Contract {
            base: AggregateRoot::new(id),
            publication_id,
            status_history: StatusHistory::new(ContractStatus::Requested),
            summaries: Vec::new(),
        })
    }

    pub fn base(&self) -> &AggregateRoot<ContractId, Event> {
        &self.base
    }

    pub fn publication_id(&self) -> &PublicationId {
        &self.publication_id
    }

    pub fn status_history(&self) -> &StatusHistory<ContractStatus> {
        &self.status_history
    }

    pub fn summaries(&self) -> &[Summary] {
        &self.summaries
    }

    pub fn approve(&mut self) -> Result<()> {
        if self.status_history().is_current(|s| match s {
            ContractStatus::Requested => true,
            _ => false,
        }) {
            self.status_history.add_status(ContractStatus::Approved);
            return Ok(());
        }
        Err(Error::new("contract", "cannot_be_approved"))
    }

    pub fn reject(&mut self) -> Result<()> {
        if self.status_history.is_current(|s| match s {
            ContractStatus::Requested => true,
            _ => false,
        }) {
            self.status_history.add_status(ContractStatus::Rejected);
            return Ok(());
        }
        Err(Error::new("contract", "cannto_be_rejected"))
    }

    pub fn request(&mut self) -> Result<()> {
        if self.status_history.is_current(|s| match s {
            ContractStatus::Requested | ContractStatus::Cancelled => true,
            _ => false,
        }) {
            self.status_history.add_status(ContractStatus::Requested);
            return Ok(());
        }
        Err(Error::new("contract", "cannot_be_requested"))
    }

    pub fn cancel(&mut self) -> Result<()> {
        if self.status_history.is_current(|s| match s {
            ContractStatus::Requested | ContractStatus::Approved => true,
            _ => false,
        }) {
            self.status_history.add_status(ContractStatus::Cancelled);
            return Ok(());
        }
        Err(Error::new("contract", "cannot_be_cancelled"))
    }
}
