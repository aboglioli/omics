use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::row::Row;
use tokio_postgres::Client;
use uuid::Uuid;

use common::error::Error;
use common::model::AggregateRoot;
use common::result::Result;
use common::sql::where_builder::WhereBuilder;

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
        let publications: i32 = row.get("publications");

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
            publications as u32,
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
    async fn find_by_id(&self, id: &AuthorId) -> Result<Author> {
        let row = self
            .client
            .query_one("SELECT * FROM users WHERE id = $1", &[&id.to_uuid()?])
            .await
            .map_err(|err| Error::not_found("author").wrap_raw(err))?;

        Author::from_row(row)
    }

    async fn search(
        &self,
        name: Option<&String>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> Result<Vec<Author>> {
        let offset = offset.unwrap_or_else(|| 0);
        let limit = limit
            .map(|l| if l <= 1000 { l } else { 1000 })
            .unwrap_or_else(|| 100);

        let (mut sql, params) = WhereBuilder::new()
            .add_param_opt(
                "(
                    LOWER(username) LIKE '%' || LOWER($$) || '%'
                    OR LOWER(name) LIKE '%' || LOWER($$) || '%'
                )",
                &name,
                name.is_some(),
            )
            .add_param_opt("created_at >= $$", &from, from.is_some())
            .add_param_opt("created_at <= $$", &to, to.is_some())
            .build();

        sql = format!(
            "SELECT * FROM users
            {}
            ORDER BY created_at ASC
            OFFSET {}
            LIMIT {}",
            sql, offset, limit,
        );

        let rows = self
            .client
            .query(&sql as &str, &params)
            .await
            .map_err(|err| Error::not_found("author").wrap_raw(err))?;

        let mut authors = Vec::new();

        for row in rows.into_iter() {
            authors.push(Author::from_row(row)?);
        }

        Ok(authors)
    }

    async fn save(&self, author: &mut Author) -> Result<()> {
        self.client
            .query_one(
                "SELECT * FROM users WHERE id = $1",
                &[&author.base().id().to_uuid()?],
            )
            .await
            .map_err(|err| Error::not_found("author").wrap_raw(err))?;

        self.client
            .execute(
                "UPDATE users
                SET
                    followers = $2
                WHERE
                    id = $1",
                &[&author.base().id().to_uuid()?, &(author.followers() as i32)],
            )
            .await
            .map_err(|err| Error::new("author", "update").wrap_raw(err))?;

        Ok(())
    }

    async fn delete(&self, id: &AuthorId) -> Result<()> {
        self.client
            .execute(
                "DELETE FROM users
                WHERE id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::new("author", "delete").wrap_raw(err))?;

        Ok(())
    }
}
