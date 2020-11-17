use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::row::Row;
use tokio_postgres::Client;

use common::error::Error;
use common::model::AggregateRoot;
use common::result::Result;

use crate::domain::role::{Permission, Name, Role, RoleId, RoleRepository};
use crate::domain::user::UserId;

impl Role {
    fn from_row(row: Row) -> Result<Self> {
        let id: String = row.get("id");
        let name: String = row.get("name");

        let permissions: Vec<Permission> = serde_json::from_value(row.get("permissions"))?;

        let created_at: DateTime<Utc> = row.get("created_at");

        Ok(Role::build(
            AggregateRoot::build(RoleId::new(id)?, created_at, None, None),
            Name::new(name)?,
            permissions,
        ))
    }
}

pub struct PostgresRoleRepository {
    client: Arc<Client>,
}

impl PostgresRoleRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgresRoleRepository { client }
    }
}

#[async_trait]
impl RoleRepository for PostgresRoleRepository {
    async fn find_all(&self) -> Result<Vec<Role>> {
        let rows = self
            .client
            .query("SELECT * FROM roles", &[])
            .await
            .map_err(|err| Error::not_found("role").wrap_raw(err))?;

        let mut roles = Vec::new();

        for row in rows.into_iter() {
            roles.push(Role::from_row(row)?);
        }

        Ok(roles)
    }

    async fn find_by_id(&self, id: &RoleId) -> Result<Role> {
        let row = self
            .client
            .query_one("SELECT * FROM roles WHERE id = $1", &[&id.value()])
            .await
            .map_err(|err| Error::not_found("role").wrap_raw(err))?;

        Role::from_row(row)
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Role> {
        let row = self
            .client
            .query_one(
                "SELECT r.* FROM users AS u
                LEFT JOIN roles r ON r.id = u.role_id
                WHERE u.id = $1",
                &[&user_id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::not_found("role").wrap_raw(err))?;

        Role::from_row(row)
    }

    async fn save(&self, role: &mut Role) -> Result<()> {
        let create = self
            .client
            .query_one(
                "SELECT * FROM roles WHERE id = $1",
                &[&role.base().id().value()],
            )
            .await
            .is_err();

        if create {
            self.client
                .execute(
                    "INSERT INTO roles(id, name, created_at)
                    VALUES ($1, $2, $3)",
                    &[
                        &role.base().id().value(),
                        &role.name(),
                        &role.base().created_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("role", "create").wrap_raw(err))?;
        } else {
            self.client
                .execute(
                    "UPDATE roles
                    SET
                        name = $2
                    WHERE
                        id = $1",
                    &[&role.base().id().value(), &role.name()],
                )
                .await
                .map_err(|err| Error::new("role", "update").wrap_raw(err))?;
        }

        Ok(())
    }
}
