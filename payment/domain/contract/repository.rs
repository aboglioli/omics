use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use common::error::Error;
use common::model::Pagination;
use common::result::Result;
use publishing::domain::publication::PublicationId;

use crate::domain::contract::{Contract, ContractId, Status};

#[async_trait]
pub trait ContractRepository: Sync + Send {
    async fn next_id(&self) -> Result<ContractId> {
        ContractId::new(Uuid::new_v4().to_string())
    }

    async fn find_by_id(&self, id: &ContractId) -> Result<Contract>;
    async fn find_by_publication_id(&self, id: &PublicationId) -> Result<Contract>;
    async fn search(
        &self,
        publication_id: Option<&PublicationId>,
        status: Option<&Status>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
        offset: Option<usize>,
        limit: Option<usize>,
        order_by: Option<&ContractOrderBy>,
    ) -> Result<Pagination<Contract>>;

    async fn save(&self, contract: &mut Contract) -> Result<()>;

    async fn delete(&self, id: &ContractId) -> Result<()>;
}

pub enum ContractOrderBy {
    Oldest,
    Newest,
}

impl FromStr for ContractOrderBy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "newest" => ContractOrderBy::Newest,
            _ => ContractOrderBy::Oldest,
        })
    }
}
