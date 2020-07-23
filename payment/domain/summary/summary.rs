use chrono::{DateTime, Utc};

use common::error::Error;
use common::model::{Entity, StatusHistory, StatusItem, ID};

use crate::domain::contract::{Contract, ContractID};
use crate::domain::summary::SummaryStatus;

type SummaryID = String;

#[derive(Debug, Clone)]
pub struct Summary {
    id: ID<SummaryID>,
    contract_id: ContractID,
    status: StatusHistory<SummaryStatus, ()>,
}

impl Summary {
    pub fn new(id: SummaryID, contract_id: ContractID) -> Result<Summary, Error> {
        Ok(Summary {
            id: ID::new(id),
            contract_id,
            status: StatusHistory::init(SummaryStatus::Open),
        })
    }

    pub fn contract_id(&self) -> &ContractID {
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

impl Entity<SummaryID> for Summary {
    fn id(&self) -> &ID<SummaryID> {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::publication::{Publication, PublicationID};
    use crate::domain::user::{User, UserID};

    #[test]
    fn create() {
        let c = Contract::new(
            ContractID::from("C007"),
            &Publication::new(
                PublicationID::from("P001"),
                &User::new(UserID::from("U055"), "User").unwrap(),
                "Pub. 1",
            )
            .unwrap(),
        )
        .unwrap();
        let s_res = Summary::new(SummaryID::from("S002"), &c);
        assert!(s_res.is_ok());

        let s = s_res.unwrap();
        assert_eq!(s.status().history().len(), 1);
    }

    #[test]
    fn statuses_ok() {
        let mut s = Summary::new(
            SummaryID::from("S005"),
            &Contract::new(ContractID::from("C012")).unwrap(),
        )
        .unwrap();

        s.ready_to_pay().unwrap();
        s.pay().unwrap();
        assert_eq!(s.status().current().unwrap().status(), &SummaryStatus::Paid);
    }

    #[test]
    fn invalid_statuses() {
        let mut s = Summary::new(
            SummaryID::from("S005"),
            &Contract::new(ContractID::from("C012")).unwrap(),
        )
        .unwrap();
        assert!(s.pay().is_err());

        s.ready_to_pay().unwrap();
        assert!(s.cancel().is_ok());
        assert!(s.pay().is_err());
        assert!(s.cancel().is_err());
        assert!(s.ready_to_pay().is_err());
    }
}
