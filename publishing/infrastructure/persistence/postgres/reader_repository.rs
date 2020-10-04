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

        let username: String = row.get("username");

        let name: Option<String> = row.get("name");
        let lastname: Option<String> = row.get("lastname");
        let profile_image: Option<String> = row.get("profile_image");

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
            username,
            name,
            lastname,
            profile_image,
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
        self.client
            .query_one(
                "SELECT * FROM users WHERE id = $1",
                &[&reader.base().id().to_uuid()?],
            )
            .await
            .map_err(|err| Error::not_found("reader").wrap_raw(err))?;

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

        Ok(())
    }

    async fn delete(&self, id: &ReaderId) -> Result<()> {
        self.client
            .execute(
                "DELETE FROM users
                WHERE id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::new("reader", "delete").wrap_raw(err))?;

        Ok(())
    }
}
