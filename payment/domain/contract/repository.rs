use async_trait::async_trait;
use uuid::Uuid;

use common::result::Result;

use crate::domain::contract::{Contract, ContractId};
use crate::domain::publication::PublicationId;

#[async_trait]
pub trait ContractRepository: Sync + Send {
    async fn next_id(&self) -> Result<ContractId> {
        ContractId::new(Uuid::new_v4().to_string())
    }

    async fn find_by_id(&self, contract_id: &ContractId) -> Result<Contract>;
    async fn find_by_publication_id(&self, publication_id: &PublicationId) -> Result<Contract>;
    async fn find_by_status(&self, status: &str) -> Result<Vec<Contract>>;

    async fn save(&self, contract: &mut Contract) -> Result<()>;
}
