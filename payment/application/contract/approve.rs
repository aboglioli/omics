use common::event::EventPublisher;
use common::result::Result;

use crate::domain::admin::{AdminId, AdminRepository};
use crate::domain::contract::{ContractId, ContractRepository};

pub struct Approve<'a, EPub, ARepo, CRepo> {
    event_pub: &'a EPub,

    admin_repo: &'a ARepo,
    contract_repo: &'a CRepo,
}

impl<'a, EPub, ARepo, CRepo> Approve<'a, EPub, ARepo, CRepo>
where
    EPub: EventPublisher,
    ARepo: AdminRepository,
    CRepo: ContractRepository,
{
    pub fn new(event_pub: &'a EPub, admin_repo: &'a ARepo, contract_repo: &'a CRepo) -> Self {
        Approve {
            event_pub,
            admin_repo,
            contract_repo,
        }
    }

    pub async fn exec(&self, admin_id: String, contract_id: String) -> Result<()> {
        let contract_id = ContractId::new(contract_id)?;
        let mut contract = self.contract_repo.find_by_id(&contract_id).await?;

        let admin_id = AdminId::new(admin_id)?;
        let admin = self.admin_repo.find_by_id(&admin_id).await?;

        contract.approve(&admin)?;

        self.contract_repo.save(&mut contract).await?;

        self.event_pub
            .publish_all(contract.base().events()?)
            .await?;

        Ok(())
    }
}
