use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::row::Row;
use tokio_postgres::Client;
use uuid::Uuid;

use common::error::Error;
use common::model::AggregateRoot;
use common::result::Result;

use crate::domain::reader::{Reader, ReaderId, ReaderRepository};

impl Reader {
    fn from_row(row: Row) -> Result<Self> {
        let id: Uuid = row.get("id");
        let subscribed: bool = row.get("subscribed");

        let created_at: DateTime<Utc> = row.get("created_at");
        let updated_at: Option<DateTime<Utc>> = row.get("updated_at");
        let deleted_at: Option<DateTime<Utc>> = row.get("deleted_at");

        Ok(Reader::build(
            AggregateRoot::build(
                ReaderId::new(id.to_string())?,
                created_at,
                updated_at,
                deleted_at,
            ),
            subscribed,
        ))
    }
}

pub struct PostgresReaderRepository {
    client: Arc<Client>,
}

impl PostgresReaderRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgresReaderRepository { client }
    }
}

#[async_trait]
impl ReaderRepository for PostgresReaderRepository {
    async fn find_by_id(&self, id: &ReaderId) -> Result<Reader> {
        let row = self
            .client
            .query_one("SELECT * FROM users WHERE id = $1", &[&id.to_uuid()?])
            .await
            .map_err(|err| Error::not_found("reader").wrap_raw(err))?;

        Reader::from_row(row)
    }

    async fn save(&self, reader: &mut Reader) -> Result<()> {
        let create = self
            .client
            .query_one(
                "SELECT * FROM users WHERE id = $1",
                &[&reader.base().id().to_uuid()?],
            )
            .await
            .is_err();

        if create {
            self.client
                .execute(
                    "INSERT INTO users(
                        id,
                        subscribed,
                        created_at,
                        updated_at,
                        deleted_at
                    ) VALUES (
                        $1,
                        $2,
                        $3,
                        $4,
                        $5
                    )",
                    &[
                        &reader.base().id().to_uuid()?,
                        &reader.is_subscribed(),
                        &reader.base().created_at(),
                        &reader.base().updated_at(),
                        &reader.base().deleted_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("reader", "create").wrap_raw(err))?;
        } else {
            self.client
                .execute(
                    "UPDATE users
                    SET
                        subscribed = $2,
                    WHERE
                        id = $1",
                    &[&reader.base().id().to_uuid()?, &reader.is_subscribed()],
                )
                .await
                .map_err(|err| Error::new("reader", "update").wrap_raw(err))?;
        }

        Ok(())
    }
}
