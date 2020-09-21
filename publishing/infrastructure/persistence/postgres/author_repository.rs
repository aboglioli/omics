use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::row::Row;
use tokio_postgres::Client;
use uuid::Uuid;

use common::error::Error;
use common::model::AggregateRoot;
use common::result::Result;

use crate::domain::author::{Author, AuthorId, AuthorRepository};

impl Author {
    fn from_row(row: Row) -> Result<Self> {
        let id: Uuid = row.get("id");

        let username: String = row.get("username");

        let name: Option<String> = row.get("name");
        let lastname: Option<String> = row.get("lastname");
        let biography: Option<String> = row.get("biography");
        let profile_image: Option<String> = row.get("profile_image");

        let followers: i32 = row.get("followers");

        let created_at: DateTime<Utc> = row.get("created_at");
        let updated_at: Option<DateTime<Utc>> = row.get("updated_at");
        let deleted_at: Option<DateTime<Utc>> = row.get("deleted_at");

        Ok(Author::build(
            AggregateRoot::build(
                AuthorId::new(id.to_string())?,
                created_at,
                updated_at,
                deleted_at,
            ),
            username,
            name,
            lastname,
            biography,
            profile_image,
            followers as u32,
        ))
    }
}

pub struct PostgresAuthorRepository {
    client: Arc<Client>,
}

impl PostgresAuthorRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgresAuthorRepository { client }
    }
}

#[async_trait]
impl AuthorRepository for PostgresAuthorRepository {
    async fn find_all(&self) -> Result<Vec<Author>> {
        let rows = self
            .client
            .query("SELECT * FROM users", &[])
            .await
            .map_err(|err| Error::not_found("author").wrap_raw(err))?;

        let mut authors = Vec::new();

        for row in rows.into_iter() {
            authors.push(Author::from_row(row)?);
        }

        Ok(authors)
    }

    async fn find_by_id(&self, id: &AuthorId) -> Result<Author> {
        let row = self
            .client
            .query_one("SELECT * FROM users WHERE id = $1", &[&id.to_uuid()?])
            .await
            .map_err(|err| Error::not_found("author").wrap_raw(err))?;

        Author::from_row(row)
    }

    async fn save(&self, author: &mut Author) -> Result<()> {
        let create = self
            .client
            .query_one(
                "SELECT * FROM users WHERE id = $1",
                &[&author.base().id().to_uuid()?],
            )
            .await
            .is_err();

        if create {
            self.client
                .execute(
                    "INSERT INTO users(
                        id,
                        followers,
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
                        &author.base().id().to_uuid()?,
                        &author.followers(),
                        &author.base().created_at(),
                        &author.base().updated_at(),
                        &author.base().deleted_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("author", "create").wrap_raw(err))?;
        } else {
            self.client
                .execute(
                    "UPDATE users
                    SET
                        followers = $2,
                    WHERE
                        id = $1",
                    &[&author.base().id().to_uuid()?, &author.followers()],
                )
                .await
                .map_err(|err| Error::new("author", "update").wrap_raw(err))?;
        }

        Ok(())
    }
}
