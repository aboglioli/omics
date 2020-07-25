use chrono::{DateTime, Utc};

use common::error::Error;
use common::model::{AggregateRoot, StatusHistory};

use crate::domain::contract::{Contract, ContractId};
use crate::domain::summary::SummaryStatus;

type SummaryId = String;

#[derive(Debug, Clone)]
pub struct Summary {
    base: AggregateRoot<SummaryId>,
    contract_id: ContractId,
    status: StatusHistory<SummaryStatus, ()>,
}

impl Summary {
    pub fn new(id: SummaryId, contract_id: ContractId) -> Result<Summary, Error> {
        Ok(Summary {
            base: AggregateRoot::new(id),
            contract_id,
            status: StatusHistory::init(SummaryStatus::Open),
        })
    }

    pub fn contract_id(&self) -> &ContractId {
        &self.contract_id
    }

    pub fn status(&self) -> &StatusHistory<SummaryStatus, ()> {
        &self.status
    }

    pub fn ready_to_pay(&mut self) -> Result<(), Error> {
        if self.status.is_current_any(&[&SummaryStatus::Open]) {
            self.status.add_status(SummaryStatus::ReadyToPay);
            return Ok(());
        }
        Err(Error::application())
    }

    pub fn pay(&mut self) -> Result<(), Error> {
        if self.status.is_current_any(&[&SummaryStatus::ReadyToPay]) {
            self.status.add_status(SummaryStatus::Paid);
            return Ok(());
        }
        Err(Error::application())
    }

    pub fn cancel(&mut self) -> Result<(), Error> {
        if self
            .status
            .is_current_any(&[&SummaryStatus::Open, &SummaryStatus::ReadyToPay])
        {
            self.status.add_status(SummaryStatus::Cancelled);
            return Ok(());
        }
        Err(Error::application())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::publication::{Publication, PublicationId};
    use crate::domain::user::{User, UserId};

    #[test]
    fn create() {
        let s_res = Summary::new(SummaryId::from("S002"), ContractId::from("C122"));
        assert!(s_res.is_ok());

        let s = s_res.unwrap();
        assert_eq!(s.status().history().len(), 1);
    }

    #[test]
    fn statuses_ok() {
        let mut s = Summary::new(SummaryId::from("S005"), ContractId::from("C623")).unwrap();

        s.ready_to_pay().unwrap();
        s.pay().unwrap();
        assert_eq!(s.status().current().unwrap().status(), &SummaryStatus::Paid);
    }

    #[test]
    fn invalid_statuses() {
        let mut s = Summary::new(SummaryId::from("S005"), ContractId::from("C62")).unwrap();
        assert!(s.pay().is_err());

        s.ready_to_pay().unwrap();
        assert!(s.cancel().is_ok());
        assert!(s.pay().is_err());
        assert!(s.cancel().is_err());
        assert!(s.ready_to_pay().is_err());
    }
}
