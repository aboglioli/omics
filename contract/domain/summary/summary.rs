use common::error::Error;
use common::event::Event;
use common::model::{AggregateRoot, StatusHistory};
use common::result::Result;

use crate::domain::contract::ContractId;
use crate::domain::summary::SummaryStatus;

type SummaryId = String;

#[derive(Debug, Clone)]
pub struct Summary {
    base: AggregateRoot<SummaryId, Event>,
    contract_id: ContractId,
    status_history: StatusHistory<SummaryStatus>,
}

impl Summary {
    pub fn new(id: SummaryId, contract_id: ContractId) -> Result<Summary> {
        Ok(Summary {
            base: AggregateRoot::new(id),
            contract_id,
            status_history: StatusHistory::new(SummaryStatus::Open),
        })
    }

    pub fn contract_id(&self) -> &ContractId {
        &self.contract_id
    }

    pub fn status_history(&self) -> &StatusHistory<SummaryStatus> {
        &self.status_history
    }

    pub fn ready_to_pay(&mut self) -> Result<()> {
        if self.status_history.is_current(|s| match s {
            SummaryStatus::Open | SummaryStatus::ReadyToPay => true,
            _ => false,
        }) {
            self.status_history.add_status(SummaryStatus::ReadyToPay);
            return Ok(());
        }
        Err(Error::new("summary", "invalid_status"))
    }

    pub fn pay(&mut self) -> Result<()> {
        if self.status_history.is_current(|s| match s {
            SummaryStatus::ReadyToPay => true,
            _ => false,
        }) {
            self.status_history.add_status(SummaryStatus::Paid);
            return Ok(());
        }
        Err(Error::new("summary", "invalid_status"))
    }

    pub fn cancel(&mut self) -> Result<()> {
        if self.status_history.is_current(|s| match s {
            SummaryStatus::Open | SummaryStatus::ReadyToPay => true,
            _ => false,
        }) {
            self.status_history.add_status(SummaryStatus::Cancelled);
            return Ok(());
        }
        Err(Error::new("summary", "invalid_status"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let s_res = Summary::new(SummaryId::from("S002"), ContractId::from("C122"));
        assert!(s_res.is_ok());

        let s = s_res.unwrap();
        assert_eq!(s.status_history().history().len(), 1);
    }

    #[test]
    fn statuses_ok() {
        let mut s = Summary::new(SummaryId::from("S005"), ContractId::from("C623")).unwrap();

        s.ready_to_pay().unwrap();
        s.pay().unwrap();
        assert_eq!(s.status_history().current().status(), &SummaryStatus::Paid);
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
