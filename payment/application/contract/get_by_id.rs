use common::result::Result;

use crate::application::dtos::ContractDto;
use crate::domain::contract::{ContractId, ContractRepository};

pub struct GetById<'a, CRepo> {
    contract_repo: &'a CRepo,
}

impl<'a, CRepo> GetById<'a, CRepo>
where
    CRepo: ContractRepository,
{
    pub fn new(contract_repo: &'a CRepo) -> Self {
        GetById { contract_repo }
    }

    pub async fn exec(&self, contract_id: String) -> Result<ContractDto> {
        let contract_id = ContractId::new(contract_id)?;
        let contract = self.contract_repo.find_by_id(&contract_id).await?;

        Ok(ContractDto::from(&contract))
    }
}
