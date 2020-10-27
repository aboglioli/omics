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

use crate::domain::donation::{Donation, DonationId, DonationOrderBy, DonationRepository, Status};
use crate::domain::payment::{Amount, Payment};

impl Donation {
    fn from_row(row: Row) -> Result<Self> {
        let id: Uuid = row.get("id");
        let author_id: Uuid = row.get("author_id");
        let reader_id: Uuid = row.get("reader_id");

        let amount: f64 = row.get("amount");
        let comment: String = row.get("comment");

        let reader_payment: Option<Payment> = serde_json::from_value(row.get("reader_payment"))?;
        let author_charge: Option<Payment> = serde_json::from_value(row.get("author_charge"))?;

        let status_history: Vec<StatusItem<Status>> =
            serde_json::from_value(row.get("status_history"))?;

        let created_at: DateTime<Utc> = row.get("created_at");
        let updated_at: Option<DateTime<Utc>> = row.get("updated_at");
        let deleted_at: Option<DateTime<Utc>> = row.get("deleted_at");

        Ok(Donation::build(
            AggregateRoot::build(
                DonationId::new(id.to_string())?,
                created_at,
                updated_at,
                deleted_at,
            ),
            UserId::new(author_id.to_string())?,
            UserId::new(reader_id.to_string())?,
            Amount::new(amount)?,
            comment,
            reader_payment,
            author_charge,
            StatusHistory::build(status_history),
        ))
    }
}

pub struct PostgresDonationRepository {
    client: Arc<Client>,
}

impl PostgresDonationRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgresDonationRepository { client }
    }
}

#[async_trait]
impl DonationRepository for PostgresDonationRepository {
    async fn find_by_id(&self, id: &DonationId) -> Result<Donation> {
        let row = self
            .client
            .query_one(
                "SELECT * FROM donations
                WHERE id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::not_found("donation").wrap_raw(err))?;

        Donation::from_row(row)
    }

    async fn search(
        &self,
        author_id: Option<&UserId>,
        reader_id: Option<&UserId>,
        status: Option<&Status>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
        offset: Option<usize>,
        limit: Option<usize>,
        order_by: Option<&DonationOrderBy>,
    ) -> Result<Pagination<Donation>> {
        let author_id = author_id.map(|id| id.to_uuid()).transpose()?;
        let reader_id = reader_id.map(|id| id.to_uuid()).transpose()?;
        let status = status.map(|s| s.to_string());

        let (sql, params) = WhereBuilder::new()
            .add_param_opt("author_id = $$", &author_id, author_id.is_some())
            .add_param_opt("reader_id = $$", &reader_id, reader_id.is_some())
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
            .query_one(&format!("SELECT COUNT(*) FROM donations") as &str, &[])
            .await
            .map_err(|err| Error::new("donation", "total").wrap_raw(err))?;
        let total: i64 = row.get(0);

        // Matching criteria
        let row = self
            .client
            .query_one(
                &format!(
                    "SELECT COUNT(*) FROM donations
                    {}",
                    sql,
                ) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::new("donation", "matching_criteria").wrap_raw(err))?;
        let matching_criteria: i64 = row.get(0);

        // Query
        let offset = offset.unwrap_or_else(|| 0);
        let limit = limit.unwrap_or_else(|| total as usize);
        let order_by = match order_by {
            Some(DonationOrderBy::Newest) => "created_at DESC",
            Some(DonationOrderBy::Amount) => "amount DESC",
            _ => "created_at ASC",
        };

        let rows = self
            .client
            .query(
                &format!(
                    "SELECT * FROM donations
                    {}
                    ORDER BY {}
                    OFFSET {}
                    LIMIT {}",
                    sql, order_by, offset, limit,
                ) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::not_found("donation").wrap_raw(err))?;

        let mut donations = Vec::new();
        for row in rows.into_iter() {
            donations.push(Donation::from_row(row)?);
        }

        Ok(
            Pagination::new(offset, limit, total as usize, matching_criteria as usize)
                .add_items(donations),
        )
    }

    async fn save(&self, donation: &mut Donation) -> Result<()> {
        let create = self
            .client
            .query_one(
                "SELECT * FROM donations WHERE id = $1",
                &[&donation.base().id().to_uuid()?],
            )
            .await
            .is_err();

        let reader_payment = serde_json::to_value(donation.reader_payment())?;
        let author_charge = serde_json::to_value(donation.author_charge())?;
        let status_history = serde_json::to_value(donation.status_history().history())?;

        if create {
            self.client
                .execute(
                    "INSERT INTO donations(
                        id,
                        author_id,
                        reader_id,
                        amount,
                        comment,
                        reader_payment,
                        author_charge,
                        status_history,
                        created_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
                    &[
                        &donation.base().id().to_uuid()?,
                        &donation.author_id().to_uuid()?,
                        &donation.reader_id().to_uuid()?,
                        &donation.amount().value(),
                        &donation.comment(),
                        &reader_payment,
                        &author_charge,
                        &status_history,
                        &donation.base().created_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("donation", "create").wrap_raw(err))?;
        } else {
            self.client
                .execute(
                    "UPDATE donations
                    SET
                        reader_payment = $2,
                        author_charge = $3,
                        status_history = $4,
                        updated_at = $5,
                        deleted_at= $6
                    WHERE
                        id = $1",
                    &[
                        &donation.base().id().to_uuid()?,
                        &reader_payment,
                        &author_charge,
                        &status_history,
                        &donation.base().updated_at(),
                        &donation.base().deleted_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("donation", "update").wrap_raw(err))?;
        }

        Ok(())
    }

    async fn delete(&self, id: &DonationId) -> Result<()> {
        self.client
            .execute(
                "DELETE FROM donations
                WHERE id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::new("donation", "delete").wrap_raw(err))?;

        Ok(())
    }
}
