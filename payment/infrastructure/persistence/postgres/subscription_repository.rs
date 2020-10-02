use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::row::Row;
use tokio_postgres::Client;
use uuid::Uuid;

use common::error::Error;
use common::result::Result;
use identity::domain::user::UserId;

use crate::domain::subscription::{Subscription, SubscriptionId, SubscriptionRepository};

impl Subscription {
    fn from_row(row: Row) -> Result<Self> {
        let id: Uuid = row.get("id");
        let user_id: Uuid = row.get("user_id");
        let plan: SubscriptionPlan = row.get("plan");
        let payments: Vec<Payment> = row.get("payments");
        let status_history: Vec<StatusItem<Status>> = row.get("status_history");

        let created_at: Datetime<Utc> = row.get("created_at");
        let updated_at: Option<DateTime<Utc>> = row.get("updated_at");
        let deleted_at: Option<DateTime<Utc>> = row.get("deleted_at");

        Ok(Subscription::build(
                AggregateRoot::build(
                    SubscriptionId::new(id.to_string())?,
                    created_at,
                    updated_at,
                    deleted_at,
                ),
                UserId::new(user_id.to_string())?,
                plan,
                payments,
                StatusHistory::build(status_history),
        ))
    }
}

pub struct PostgresSubscriptionRepository {
    client: Arc<Client>,
}

impl PostgresSubscriptionRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgresSubscriptionRepository { client }
    }
}

#[async_trait]
impl SubscriptionRepository for PostgresSubscriptionRepository {
    async fn find_by_id(&self, id: &SubscriptionId) -> Result<Subscription> {
        let row = self
            .client
            .query_one(
                "SELECT * FROM subscriptions
                WHERE id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::not_found("subscription").wrap_raw(err))?;

        Subscription::from_row(row)
    }
}
