use std::sync::Arc;

use async_trait::async_trait;

use tokio_postgres::Client;

use crate::cache::Cache;
use crate::error::Error;
use crate::result::Result;

pub struct PostgresCache {
    table: String,
    client: Arc<Client>,
}

impl PostgresCache {
    pub fn new<S: Into<String>>(table: S, client: Arc<Client>) -> Self {
        PostgresCache {
            table: table.into(),
            client,
        }
    }
}

#[async_trait]
impl Cache<String, String> for PostgresCache {
    async fn get(&self, k: &String) -> Option<String> {
        let row = self
            .client
            .query_one(
                &format!(
                    "SELECT value FROM {}
                    WHERE key = $1",
                    self.table,
                ) as &str,
                &[&k],
            )
            .await
            .ok();

        if let Some(row) = row {
            let value: Option<String> = row.try_get("value").ok();

            if value.is_some() {
                return value;
            }
        }

        None
    }

    async fn set(&self, k: String, v: String) -> Result<()> {
        let create = self.get(&k).await.is_none();

        if create {
            self.client
                .execute(
                    &format!(
                        "INSERT INTO {}(key, value)
                        VALUES ($1, $2)",
                        self.table,
                    ) as &str,
                    &[&k, &v],
                )
                .await
                .map_err(|err| Error::new(&self.table as &str, "create").wrap_raw(err))?;
        } else {
            self.client
                .execute(
                    &format!(
                        "UPDATE {}
                        SET
                            value = $2
                        WHERE key = $1",
                        self.table,
                    ) as &str,
                    &[&k, &v],
                )
                .await
                .map_err(|err| Error::new(&self.table as &str, "update").wrap_raw(err))?;
        }

        Ok(())
    }

    async fn delete(&self, k: &String) -> Result<()> {
        self.client
            .execute(
                &format!(
                    "DELETE FROM {}
                    WHERE key = $1",
                    self.table,
                ) as &str,
                &[&k],
            )
            .await
            .map_err(|err| Error::new(&self.table as &str, "delete").wrap_raw(err))?;

        Ok(())
    }
}
