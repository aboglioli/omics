use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::row::Row;
use tokio_postgres::Client;
use uuid::Uuid;

use common::error::Error;
use common::model::{AggregateRoot, StatusHistory, StatusItem};
use common::result::Result;
use common::sql::where_builder::WhereBuilder;
use publishing::domain::publication::PublicationId;

use crate::domain::contract::{Contract, ContractId, ContractRepository, Status, Summary};
use crate::domain::payment::Payment;

impl Contract {
    fn from_row(row: Row) -> Result<Self> {
        let id: Uuid = row.get("id");
        let publication_id: Uuid = row.get("publication_id");

        let summaries: Vec<Summary> = serde_json::from_value(row.get("summaries"))?;
        let payments: Vec<Payment> = serde_json::from_value(row.get("payments"))?;

        let status_history: Vec<StatusItem<Status>> =
            serde_json::from_value(row.get("status_history"))?;

        let created_at: DateTime<Utc> = row.get("created_at");
        let updated_at: Option<DateTime<Utc>> = row.get("updated_at");
        let deleted_at: Option<DateTime<Utc>> = row.get("deleted_at");

        Ok(Contract::build(
            AggregateRoot::build(
                ContractId::new(id.to_string())?,
                created_at,
                updated_at,
                deleted_at,
            ),
            PublicationId::new(publication_id.to_string())?,
            summaries,
            payments,
            StatusHistory::build(status_history),
        ))
    }
}

pub struct PostgresContractRepository {
    client: Arc<Client>,
}

impl PostgresContractRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgresContractRepository { client }
    }
}

#[async_trait]
impl ContractRepository for PostgresContractRepository {
    async fn find_by_id(&self, id: &ContractId) -> Result<Contract> {
        let row = self
            .client
            .query_one(
                "SELECT * FROM contracts
                WHERE id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::not_found("contract").wrap_raw(err))?;

        Contract::from_row(row)
    }

    async fn search(
        &self,
        publication_id: Option<&PublicationId>,
        status: Option<&String>,
    ) -> Result<Vec<Contract>> {
        let publication_id = publication_id.map(|id| id.to_uuid()).transpose()?;

        let (sql, params) = WhereBuilder::new()
            .add_param_opt(
                "publication_id = $$",
                &publication_id,
                publication_id.is_some(),
            )
            .add_param_opt(
                "status_history->-1->>'status' = $$",
                &status,
                status.is_some(),
            )
            .build();

        let rows = self
            .client
            .query(
                &format!(
                    "SELECT * FROM contracts
                    {}
                    ORDER BY created_at ASC",
                    sql,
                ) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::not_found("contract").wrap_raw(err))?;

        let mut contracts = Vec::new();
        for row in rows.into_iter() {
            contracts.push(Contract::from_row(row)?);
        }

        Ok(contracts)
    }

    async fn save(&self, contract: &mut Contract) -> Result<()> {
        let create = self
            .client
            .query_one(
                "SELECT * FROM contracts WHERE id = $1",
                &[&contract.base().id().to_uuid()?],
            )
            .await
            .is_err();

        let summaries = serde_json::to_value(contract.summaries())?;
        let payments = serde_json::to_value(contract.payments())?;
        let status_history = serde_json::to_value(contract.status_history().history())?;

        if create {
            self.client
                .execute(
                    "INSERT INTO contracts(
                        id,
                        publication_id,
                        summaries,
                        payments,
                        status_history,
                        created_at
                    ) VALUES ($1, $2, $3, $4, $5, $6)",
                    &[
                        &contract.base().id().to_uuid()?,
                        &contract.publication_id().to_uuid()?,
                        &summaries,
                        &payments,
                        &status_history,
                        &contract.base().created_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("contract", "create").wrap_raw(err))?;
        } else {
            self.client
                .execute(
                    "UPDATE contracts
                    SET
                        summaries = $2,
                        payments = $3,
                        status_history = $4,
                        updated_at = $5,
                        deleted_at= $6
                    WHERE
                        id = $1",
                    &[
                        &contract.base().id().to_uuid()?,
                        &summaries,
                        &payments,
                        &status_history,
                        &contract.base().updated_at(),
                        &contract.base().deleted_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("contract", "update").wrap_raw(err))?;
        }

        Ok(())
    }

    async fn delete(&self, id: &ContractId) -> Result<()> {
        self.client
            .execute(
                "DELETE FROM contracts
                WHERE id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::new("contract", "delete").wrap_raw(err))?;

        Ok(())
    }
}
