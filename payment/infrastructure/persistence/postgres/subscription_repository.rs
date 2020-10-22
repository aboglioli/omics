use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::row::Row;
use tokio_postgres::Client;
use uuid::Uuid;

use common::error::Error;
use common::model::{AggregateRoot, Pagination, StatusHistory, StatusItem};
use common::result::Result;
use common::sql::where_builder::WhereBuilder;
use identity::domain::user::UserId;

use crate::domain::payment::Payment;
use crate::domain::plan::PlanId;
use crate::domain::subscription::{
    Status, Subscription, SubscriptionId, SubscriptionOrderBy, SubscriptionPlan,
    SubscriptionRepository,
};

impl Subscription {
    fn from_row(row: Row) -> Result<Self> {
        let id: Uuid = row.get("id");
        let user_id: Uuid = row.get("user_id");
        let plan: SubscriptionPlan = serde_json::from_value(row.get("plan"))?;
        let payments: Vec<Payment> = serde_json::from_value(row.get("payments"))?;
        let status_history: Vec<StatusItem<Status>> =
            serde_json::from_value(row.get("status_history"))?;

        let created_at: DateTime<Utc> = row.get("created_at");
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

    async fn find_by_user_id(&self, id: &UserId) -> Result<Subscription> {
        let row = self
            .client
            .query_one(
                "SELECT * FROM subscriptions
                WHERE user_id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::not_found("subscription").wrap_raw(err))?;

        Subscription::from_row(row)
    }

    async fn search(
        &self,
        user_id: Option<&UserId>,
        plan_id: Option<&PlanId>,
        status: Option<&Status>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
        offset: Option<usize>,
        limit: Option<usize>,
        order_by: Option<&SubscriptionOrderBy>,
    ) -> Result<Pagination<Subscription>> {
        let user_id = user_id.map(|id| id.to_uuid()).transpose()?;
        let plan_id = plan_id.map(|id| id.value()); // TODO: use
        let status = status.map(|s| s.to_string());

        let (sql, params) = WhereBuilder::new()
            .add_param_opt("user_id = $$", &user_id, user_id.is_some())
            .add_param_opt(
                "status_history->-1->>'status' = $$",
                &status,
                status.is_some(),
            )
            .add_param_opt("created_at >= $$", &from, from.is_some())
            .add_param_opt("created_at <= $$", &to, to.is_some())
            .build();

        // Total
        let row = self
            .client
            .query_one(&format!("SELECT COUNT(*) FROM subscriptions") as &str, &[])
            .await
            .map_err(|err| Error::new("subscription", "total").wrap_raw(err))?;
        let total: i64 = row.get(0);

        // Matching criteria
        let row = self
            .client
            .query_one(
                &format!(
                    "SELECT COUNT(*) FROM subscriptions
                    {}",
                    sql,
                ) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::new("subscription", "matching_criteria").wrap_raw(err))?;
        let matching_criteria: i64 = row.get(0);
        //
        // Query
        let offset = offset.unwrap_or_else(|| 0);
        let limit = limit.unwrap_or_else(|| total as usize);
        let order_by = match order_by {
            Some(SubscriptionOrderBy::Newest) => "created_at DESC",
            _ => "created_at ASC",
        };

        let rows = self
            .client
            .query(
                &format!(
                    "SELECT * FROM subscriptions
                    {}
                    ORDER BY {}
                    OFFSET {}
                    LIMIT {}",
                    sql, order_by, offset, limit,
                ) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::not_found("subscription").wrap_raw(err))?;

        let mut subscriptions = Vec::new();
        for row in rows.into_iter() {
            subscriptions.push(Subscription::from_row(row)?);
        }

        Ok(
            Pagination::new(offset, limit, total as usize, matching_criteria as usize)
                .add_items(subscriptions),
        )
    }

    async fn save(&self, subscription: &mut Subscription) -> Result<()> {
        let create = self
            .client
            .query_one(
                "SELECT * FROM subscriptions WHERE id = $1",
                &[&subscription.base().id().to_uuid()?],
            )
            .await
            .is_err();

        let plan = serde_json::to_value(subscription.plan())?;
        let payments = serde_json::to_value(subscription.payments())?;
        let status_history = serde_json::to_value(subscription.status_history().history())?;

        if create {
            self.client
                .execute(
                    "INSERT INTO subscriptions(
                        id,
                        user_id,
                        plan,
                        payments,
                        status_history,
                        created_at
                    ) VALUES ($1, $2, $3, $4, $5, $6)",
                    &[
                        &subscription.base().id().to_uuid()?,
                        &subscription.user_id().to_uuid()?,
                        &plan,
                        &payments,
                        &status_history,
                        &subscription.base().created_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("subscription", "create").wrap_raw(err))?;
        } else {
            self.client
                .execute(
                    "UPDATE subscriptions
                    SET
                        user_id = $2,
                        plan = $3,
                        payments = $4,
                        status_history = $5,
                        updated_at = $6,
                        deleted_at = $7
                    WHERE
                        id = $1",
                    &[
                        &subscription.base().id().to_uuid()?,
                        &subscription.user_id().to_uuid()?,
                        &plan,
                        &payments,
                        &status_history,
                        &subscription.base().updated_at(),
                        &subscription.base().deleted_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("subscription", "update").wrap_raw(err))?;
        }

        Ok(())
    }

    async fn delete(&self, id: &SubscriptionId) -> Result<()> {
        self.client
            .execute(
                "DELETE FROM subscriptions
                WHERE id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::new("subscription", "delete").wrap_raw(err))?;

        Ok(())
    }
}
