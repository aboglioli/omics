use async_trait::async_trait;

use common::result::Result;

use crate::domain::contract::{Contract, ContractId};
use crate::domain::publication::PublicationId;

#[async_trait]
pub trait ContractRepository {
    async fn next_id(&self) -> Result<ContractId>;

    async fn find_by_id(&self, contract_id: &ContractId) -> Result<Contract>;
    async fn find_by_publication_id(&self, publication_id: &PublicationId) -> Result<Contract>;
    async fn find_by_status(&self, status: &str) -> Result<Vec<Contract>>;

    async fn save(&self, contract: &mut Contract) -> Result<()>;
}
