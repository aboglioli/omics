use async_trait::async_trait;

use common::result::Result;

use crate::domain::contract::{Contract, ContractId};

#[async_trait]
pub trait ContractRepository {
    async fn next_id(&self) -> Result<ContractId>;

    async fn find_by_id(&self, contract_id: &ContractId) -> Result<Contract>;

    async fn save(&self, contract: &mut Contract) -> Result<()>;
}
