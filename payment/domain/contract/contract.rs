use common::error::Error;
use common::model::{Entity, StatusHistory, ID};

use crate::domain::contract::ContractStatus;
use crate::domain::publication::{Publication, PublicationID};
use crate::domain::summary::Summary;

pub type ContractID = String;

pub struct Contract {
    id: ID<ContractID>,
    publication_id: PublicationID,
    status: StatusHistory<ContractStatus, String>,
    summaries: Vec<Summary>,
}

impl Contract {
    pub fn new(id: ContractID, publication: &Publication) -> Result<Contract, Error> {
        Ok(Contract {
            id: ID::new(id),
            publication_id: publication.id().value(),
            status: StatusHistory::init(ContractStatus::Requested),
            summaries: Vec::new(),
        })
    }

    pub fn publication(&self) -> &PublicationID {
        &self.publication_id
    }

    pub fn status(&self) -> &StatusHistory<ContractStatus, String> {
        &self.status
    }

    pub fn summaries(&self) -> &[Summary] {
        &self.summaries
    }

    pub fn approve(&mut self) -> Result<(), Error> {
        if self.status().is_current_any(&[&ContractStatus::Requested]) {
            self.status.add_status(ContractStatus::Approved);
            return Ok(());
        }
        Err(Error::application())
    }

    pub fn reject(&mut self) -> Result<(), Error> {
        if self.status.is_current_any(&[&ContractStatus::Requested]) {
            self.status.add_status(ContractStatus::Rejected);
            return Ok(());
        }
        Err(Error::application())
    }

    pub fn request(&mut self) -> Result<(), Error> {
        if self
            .status
            .is_current_any(&[&ContractStatus::Rejected, &ContractStatus::Cancelled])
        {
            self.status.add_status(ContractStatus::Requested);
            return Ok(());
        }
        Err(Error::application())
    }

    pub fn cancel(&mut self) -> Result<(), Error> {
        if self.status.is_current_any(&[
            &ContractStatus::Requested,
            &ContractStatus::Approved,
            &ContractStatus::Requested,
        ]) {
            self.status.add_status(ContractStatus::Cancelled);
            return Ok(());
        }
        Err(Error::application())
    }
}

impl Entity<ContractID> for Contract {
    fn id(&self) -> &ID<ContractID> {
        &self.id
    }
}
