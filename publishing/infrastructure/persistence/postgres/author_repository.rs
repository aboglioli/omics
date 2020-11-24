use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::row::Row;
use tokio_postgres::Client;
use uuid::Uuid;

use common::error::Error;
use common::model::{AggregateRoot, Pagination};
use common::result::Result;
use common::sql::where_builder::WhereBuilder;

use crate::domain::author::{Author, AuthorId, AuthorOrderBy, AuthorRepository};

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
        publications_gt: Option<u32>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
        offset: Option<usize>,
        limit: Option<usize>,
        order_by: Option<&AuthorOrderBy>,
    ) -> Result<Pagination<Author>> {
        let publications_gt = publications_gt.map(|publications_gt| publications_gt as i32);

        let (sql, params) = WhereBuilder::new()
            .add_param_opt(
                "(
                    LOWER(username) LIKE '%' || LOWER($$) || '%'
                    OR LOWER(name) LIKE '%' || LOWER($$) || '%'
                    OR LOWER(lastname) LIKE '%' || LOWER($$) || '%'
                    OR LOWER(CONCAT(name, ' ', lastname)) LIKE '%' || LOWER($$) || '%'
                )",
                &name,
                name.is_some(),
            )
            .add_param_opt(
                "publications >= $$",
                &publications_gt,
                publications_gt.is_some(),
            )
            .add_param_opt("created_at >= $$", &from, from.is_some())
            .add_param_opt("created_at <= $$", &to, to.is_some())
            .build();

        // Total
        let row = self
            .client
            .query_one(&format!("SELECT COUNT(*) FROM users") as &str, &[])
            .await
            .map_err(|err| Error::new("author", "total").wrap_raw(err))?;
        let total: i64 = row.get(0);

        // Matching criteria
        let row = self
            .client
            .query_one(
                &format!(
                    "SELECT COUNT(*) FROM users
                    {}",
                    sql,
                ) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::new("author", "matching_criteria").wrap_raw(err))?;
        let matching_criteria: i64 = row.get(0);

        // Query
        let offset = offset.unwrap_or_else(|| 0);
        let limit = limit.unwrap_or_else(|| total as usize);
        let order_by = match order_by {
            Some(AuthorOrderBy::Newest) => "created_at DESC",
            Some(AuthorOrderBy::Followers) => "followers DESC",
            Some(AuthorOrderBy::Publications) => "publications DESC",
            _ => "created_at ASC",
        };

        let rows = self
            .client
            .query(
                &format!(
                    "SELECT * FROM users
                    {}
                    ORDER BY {}
                    OFFSET {}
                    LIMIT {}",
                    sql, order_by, offset, limit,
                ) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::not_found("author").wrap_raw(err))?;

        let mut authors = Vec::new();
        for row in rows.into_iter() {
            authors.push(Author::from_row(row)?);
        }

        Ok(
            Pagination::new(offset, limit, total as usize, matching_criteria as usize)
                .add_items(authors),
        )
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
                    followers = $2,
                    publications = $3
                WHERE
                    id = $1",
                &[
                    &author.base().id().to_uuid()?,
                    &(author.followers() as i32),
                    &(author.publications() as i32),
                ],
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
