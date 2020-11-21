use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::row::Row;
use tokio_postgres::Client;

use common::error::Error;
use common::result::Result;

use crate::domain::role::{Permission, PermissionRepository};

impl Permission {
    fn from_row(row: Row) -> Result<Self> {
        let id: String = row.get("id");
        let name: String = row.get("name");

        Permission::new(id, name)
    }
}

pub struct PostgresPermissionRepository {
    client: Arc<Client>,
}

impl PostgresPermissionRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgresPermissionRepository { client }
    }
}

#[async_trait]
impl PermissionRepository for PostgresPermissionRepository {
    async fn find_all(&self) -> Result<Vec<Permission>> {
        let rows = self
            .client
            .query("SELECT * FROM permissions", &[])
            .await
            .map_err(|err| Error::not_found("permission").wrap_raw(err))?;

        let mut permissions = Vec::new();

        for row in rows.into_iter() {
            permissions.push(Permission::from_row(row)?);
        }

        Ok(permissions)
    }
}
