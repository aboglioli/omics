use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::row::Row;
use tokio_postgres::Client;

use common::error::Error;
use common::model::AggregateRoot;
use common::result::Result;

use crate::domain::plan::{Plan, PlanId, PlanRepository, Price};

impl Plan {
    fn from_row(row: Row) -> Result<Self> {
        let id: String = row.get("id");
        let price: f64 = row.get("price");

        let created_at: DateTime<Utc> = row.get("created_at");
        let updated_at: Option<DateTime<Utc>> = row.get("updated_at");
        let deleted_at: Option<DateTime<Utc>> = row.get("deleted_at");

        Ok(Plan::build(
            AggregateRoot::build(PlanId::new(id)?, created_at, updated_at, deleted_at),
            Price::new(price)?,
        ))
    }
}

pub struct PostgresPlanRepository {
    client: Arc<Client>,
}

impl PostgresPlanRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgresPlanRepository { client }
    }
}

#[async_trait]
impl PlanRepository for PostgresPlanRepository {
    async fn find_all(&self) -> Result<Vec<Plan>> {
        let rows = self
            .client
            .query("SELECT * FROM plans", &[])
            .await
            .map_err(|err| Error::not_found("plan").wrap_raw(err))?;

        let mut plans = Vec::new();
        for row in rows.into_iter() {
            plans.push(Plan::from_row(row)?);
        }

        Ok(plans)
    }

    async fn find_by_id(&self, id: &PlanId) -> Result<Plan> {
        let row = self
            .client
            .query_one("SELECT * FROM plans WHERE id = $1", &[&id.value()])
            .await
            .map_err(|err| Error::not_found("plan").wrap_raw(err))?;

        Plan::from_row(row)
    }

    async fn save(&self, plan: &mut Plan) -> Result<()> {
        let create = self
            .client
            .query_one(
                "SELECT * FROM plans WHERE id = $1",
                &[&plan.base().id().value()],
            )
            .await
            .is_err();

        if create {
            self.client
                .execute(
                    "INSERT INTO plans(
                        id,
                        price,
                        created_at
                    ) VALUES ($1, $2, $3)",
                    &[
                        &plan.base().id().value(),
                        &plan.price().value(),
                        &plan.base().created_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("plan", "create").wrap_raw(err))?;
        } else {
            self.client
                .execute(
                    "UPDATE plans
                    SET
                        price = $2,
                        updated_at = $5,
                        deleted_at= $6
                    WHERE
                        id = $1",
                    &[
                        &plan.base().id().value(),
                        &plan.base().updated_at(),
                        &plan.base().deleted_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("plan", "update").wrap_raw(err))?;
        }

        Ok(())
    }

    async fn delete(&self, id: &PlanId) -> Result<()> {
        self.client
            .execute(
                "DELETE FROM plans
                WHERE id = $1",
                &[&id.value()],
            )
            .await
            .map_err(|err| Error::new("plan", "delete").wrap_raw(err))?;

        Ok(())
    }
}
