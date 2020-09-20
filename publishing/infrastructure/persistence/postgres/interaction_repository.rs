use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use tokio_postgres::Client;
use uuid::Uuid;

use common::error::Error;
use common::model::AggregateRoot;
use common::result::Result;
use common::sql;

use crate::domain::author::AuthorId;
use crate::domain::collection::CollectionId;
use crate::domain::interaction::{
    CollectionFavorite, Comment, Follow, InteractionRepository, Like, PublicationFavorite,
    ReaderAuthorId, ReaderCollectionId, ReaderPublicationId, Reading, Review, Stars, View,
};
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

pub struct PostgresInteractionRepository {
    client: Arc<Client>,
}

impl PostgresInteractionRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgresInteractionRepository { client }
    }
}

#[async_trait]
impl InteractionRepository for PostgresInteractionRepository {
    async fn find_views(
        &self,
        reader_id: Option<&ReaderId>,
        publication_id: Option<&PublicationId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<View>> {
        let reader_id = reader_id.map(|id| id.to_uuid()).transpose()?;
        let publication_id = publication_id.map(|id| id.to_uuid()).transpose()?;

        let (sql, params) = sql::WhereBuilder::new()
            .add_param_opt("reader_id = $$", &reader_id, reader_id.is_some())
            .add_param_opt(
                "publication_id = $$",
                &publication_id,
                publication_id.is_some(),
            )
            .add_param_opt("from >= $$", &from, from.is_some())
            .add_param_opt("to <= $$", &to, to.is_some())
            .build();

        let rows = self
            .client
            .query(
                &format!("SELECT * FROM views WHERE {}", sql) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::not_found("view").wrap_raw(err))?;

        let mut views = Vec::new();
        for row in rows.into_iter() {
            let reader_id: Uuid = row.get("reader_id");
            let publication_id: Uuid = row.get("publication_id");
            let datetime: DateTime<Utc> = row.get("datetime");
            let unique: bool = row.get("is_unique");

            views.push(View::build(
                AggregateRoot::build(
                    ReaderPublicationId::new(
                        ReaderId::new(reader_id.to_string())?,
                        PublicationId::new(publication_id.to_string())?,
                    )?,
                    datetime,
                    None,
                    None,
                ),
                unique,
            ));
        }

        Ok(views)
    }

    async fn find_readings(
        &self,
        reader_id: Option<&ReaderId>,
        publication_id: Option<&PublicationId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Reading>> {
        let reader_id = reader_id.map(|id| id.to_uuid()).transpose()?;
        let publication_id = publication_id.map(|id| id.to_uuid()).transpose()?;

        let (sql, params) = sql::WhereBuilder::new()
            .add_param_opt("reader_id = $$", &reader_id, reader_id.is_some())
            .add_param_opt(
                "publication_id = $$",
                &publication_id,
                publication_id.is_some(),
            )
            .add_param_opt("from >= $$", &from, from.is_some())
            .add_param_opt("to <= $$", &to, to.is_some())
            .build();

        let rows = self
            .client
            .query(
                &format!("SELECT * FROM readings WHERE {}", sql) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::not_found("reading").wrap_raw(err))?;

        let mut readings = Vec::new();
        for row in rows.into_iter() {
            let reader_id: Uuid = row.get("reader_id");
            let publication_id: Uuid = row.get("publication_id");
            let datetime: DateTime<Utc> = row.get("datetime");

            readings.push(Reading::build(AggregateRoot::build(
                ReaderPublicationId::new(
                    ReaderId::new(reader_id.to_string())?,
                    PublicationId::new(publication_id.to_string())?,
                )?,
                datetime,
                None,
                None,
            )));
        }

        Ok(readings)
    }

    async fn find_likes(
        &self,
        reader_id: Option<&ReaderId>,
        publication_id: Option<&PublicationId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Like>> {
        let reader_id = reader_id.map(|id| id.to_uuid()).transpose()?;
        let publication_id = publication_id.map(|id| id.to_uuid()).transpose()?;

        let (sql, params) = sql::WhereBuilder::new()
            .add_param_opt("reader_id = $$", &reader_id, reader_id.is_some())
            .add_param_opt(
                "publication_id = $$",
                &publication_id,
                publication_id.is_some(),
            )
            .add_param_opt("from >= $$", &from, from.is_some())
            .add_param_opt("to <= $$", &to, to.is_some())
            .build();

        let rows = self
            .client
            .query(
                &format!("SELECT * FROM likes WHERE {}", sql) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::not_found("like").wrap_raw(err))?;

        let mut likes = Vec::new();
        for row in rows.into_iter() {
            let reader_id: Uuid = row.get("reader_id");
            let publication_id: Uuid = row.get("publication_id");
            let datetime: DateTime<Utc> = row.get("datetime");

            likes.push(Like::build(AggregateRoot::build(
                ReaderPublicationId::new(
                    ReaderId::new(reader_id.to_string())?,
                    PublicationId::new(publication_id.to_string())?,
                )?,
                datetime,
                None,
                None,
            )));
        }

        Ok(likes)
    }

    async fn find_reviews(
        &self,
        reader_id: Option<&ReaderId>,
        publication_id: Option<&PublicationId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Review>> {
        let reader_id = reader_id.map(|id| id.to_uuid()).transpose()?;
        let publication_id = publication_id.map(|id| id.to_uuid()).transpose()?;

        let (sql, params) = sql::WhereBuilder::new()
            .add_param_opt("reader_id = $$", &reader_id, reader_id.is_some())
            .add_param_opt(
                "publication_id = $$",
                &publication_id,
                publication_id.is_some(),
            )
            .add_param_opt("from >= $$", &from, from.is_some())
            .add_param_opt("to <= $$", &to, to.is_some())
            .build();

        let rows = self
            .client
            .query(
                &format!("SELECT * FROM reviews WHERE {}", sql) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::not_found("review").wrap_raw(err))?;

        let mut reviews = Vec::new();
        for row in rows.into_iter() {
            let reader_id: Uuid = row.get("reader_id");
            let publication_id: Uuid = row.get("publication_id");
            let datetime: DateTime<Utc> = row.get("datetime");

            let stars: u32 = row.get("stars");
            let comment: String = row.get("comment");

            reviews.push(Review::build(
                AggregateRoot::build(
                    ReaderPublicationId::new(
                        ReaderId::new(reader_id.to_string())?,
                        PublicationId::new(publication_id.to_string())?,
                    )?,
                    datetime,
                    None,
                    None,
                ),
                Stars::new(stars as u8)?,
                Comment::new(comment)?,
            ));
        }

        Ok(reviews)
    }

    async fn find_publication_favorites(
        &self,
        reader_id: Option<&ReaderId>,
        publication_id: Option<&PublicationId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<PublicationFavorite>> {
        let reader_id = reader_id.map(|id| id.to_uuid()).transpose()?;
        let publication_id = publication_id.map(|id| id.to_uuid()).transpose()?;

        let (sql, params) = sql::WhereBuilder::new()
            .add_param_opt("reader_id = $$", &reader_id, reader_id.is_some())
            .add_param_opt(
                "publication_id = $$",
                &publication_id,
                publication_id.is_some(),
            )
            .add_param_opt("from >= $$", &from, from.is_some())
            .add_param_opt("to <= $$", &to, to.is_some())
            .build();

        let rows = self
            .client
            .query(
                &format!("SELECT * FROM publication_favorites WHERE {}", sql) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::not_found("favorite").wrap_raw(err))?;

        let mut favorites = Vec::new();
        for row in rows.into_iter() {
            let reader_id: Uuid = row.get("reader_id");
            let publication_id: Uuid = row.get("publication_id");
            let datetime: DateTime<Utc> = row.get("datetime");

            favorites.push(PublicationFavorite::build(AggregateRoot::build(
                ReaderPublicationId::new(
                    ReaderId::new(reader_id.to_string())?,
                    PublicationId::new(publication_id.to_string())?,
                )?,
                datetime,
                None,
                None,
            )));
        }

        Ok(favorites)
    }

    async fn find_collection_favorites(
        &self,
        reader_id: Option<&ReaderId>,
        collection_id: Option<&CollectionId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<CollectionFavorite>> {
        let reader_id = reader_id.map(|id| id.to_uuid()).transpose()?;
        let collection_id = collection_id.map(|id| id.to_uuid()).transpose()?;

        let (sql, params) = sql::WhereBuilder::new()
            .add_param_opt("reader_id = $$", &reader_id, reader_id.is_some())
            .add_param_opt(
                "collection_id = $$",
                &collection_id,
                collection_id.is_some(),
            )
            .add_param_opt("from >= $$", &from, from.is_some())
            .add_param_opt("to <= $$", &to, to.is_some())
            .build();

        let rows = self
            .client
            .query(
                &format!("SELECT * FROM collection_favorites WHERE {}", sql) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::not_found("favorite").wrap_raw(err))?;

        let mut favorites = Vec::new();
        for row in rows.into_iter() {
            let reader_id: Uuid = row.get("reader_id");
            let collection_id: Uuid = row.get("collection_id");
            let datetime: DateTime<Utc> = row.get("datetime");

            favorites.push(CollectionFavorite::build(AggregateRoot::build(
                ReaderCollectionId::new(
                    ReaderId::new(reader_id.to_string())?,
                    CollectionId::new(collection_id.to_string())?,
                )?,
                datetime,
                None,
                None,
            )));
        }

        Ok(favorites)
    }

    async fn find_follows(
        &self,
        reader_id: Option<&ReaderId>,
        author_id: Option<&AuthorId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Follow>> {
        let reader_id = reader_id.map(|id| id.to_uuid()).transpose()?;
        let author_id = author_id.map(|id| id.to_uuid()).transpose()?;

        let (sql, params) = sql::WhereBuilder::new()
            .add_param_opt("reader_id = $$", &reader_id, reader_id.is_some())
            .add_param_opt("author_id = $$", &author_id, author_id.is_some())
            .add_param_opt("from >= $$", &from, from.is_some())
            .add_param_opt("to <= $$", &to, to.is_some())
            .build();

        let rows = self
            .client
            .query(
                &format!("SELECT * FROM follows WHERE {}", sql) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::not_found("follow").wrap_raw(err))?;

        let mut follows = Vec::new();
        for row in rows.into_iter() {
            let reader_id: Uuid = row.get("reader_id");
            let author_id: Uuid = row.get("author_id");
            let datetime: DateTime<Utc> = row.get("datetime");

            follows.push(Follow::build(AggregateRoot::build(
                ReaderAuthorId::new(
                    ReaderId::new(reader_id.to_string())?,
                    AuthorId::new(author_id.to_string())?,
                )?,
                datetime,
                None,
                None,
            )));
        }

        Ok(follows)
    }

    async fn save_view(&self, view: &mut View) -> Result<()> {
        self.client
            .execute(
                "INSERT INTO views(reader_id, publication_id, datetime, is_unique)
                VALUES ($1, $2, $3, $4)",
                &[
                    &view.base().id().reader_id().to_uuid()?,
                    &view.base().id().publication_id().to_uuid()?,
                    &view.base().created_at(),
                    &view.is_unique(),
                ],
            )
            .await
            .map_err(|err| Error::new("view", "create").wrap_raw(err))?;

        Ok(())
    }

    async fn save_reading(&self, reading: &mut Reading) -> Result<()> {
        self.client
            .execute(
                "INSERT INTO readings(reader_id, publication_id, datetime)
                VALUES ($1, $2, $3)",
                &[
                    &reading.base().id().reader_id().to_uuid()?,
                    &reading.base().id().publication_id().to_uuid()?,
                    &reading.base().created_at(),
                ],
            )
            .await
            .map_err(|err| Error::new("reading", "create").wrap_raw(err))?;

        Ok(())
    }

    async fn save_like(&self, like: &mut Like) -> Result<()> {
        self.client
            .execute(
                "INSERT INTO likes(reader_id, publication_id, datetime)
                VALUES ($1, $2, $3)",
                &[
                    &like.base().id().reader_id().to_uuid()?,
                    &like.base().id().publication_id().to_uuid()?,
                    &like.base().created_at(),
                ],
            )
            .await
            .map_err(|err| Error::new("like", "create").wrap_raw(err))?;

        Ok(())
    }

    async fn save_review(&self, review: &mut Review) -> Result<()> {
        self.client
            .execute(
                "INSERT INTO reviews(reader_id, publication_id, datetime, stars, comment)
                VALUES ($1, $2, $3, $4, $5)",
                &[
                    &review.base().id().reader_id().to_uuid()?,
                    &review.base().id().publication_id().to_uuid()?,
                    &review.base().created_at(),
                    &(review.stars().value() as u32),
                    &review.comment().value(),
                ],
            )
            .await
            .map_err(|err| Error::new("review", "create").wrap_raw(err))?;

        Ok(())
    }

    async fn save_publication_favorite(&self, favorite: &mut PublicationFavorite) -> Result<()> {
        self.client
            .execute(
                "INSERT INTO publication_favorites(reader_id, publication_id, datetime)
                VALUES ($1, $2, $3)",
                &[
                    &favorite.base().id().reader_id().to_uuid()?,
                    &favorite.base().id().publication_id().to_uuid()?,
                    &favorite.base().created_at(),
                ],
            )
            .await
            .map_err(|err| Error::new("publication_favorite", "create").wrap_raw(err))?;

        Ok(())
    }

    async fn save_collection_favorite(&self, favorite: &mut CollectionFavorite) -> Result<()> {
        self.client
            .execute(
                "INSERT INTO collection_favorites(reader_id, collection_id, datetime)
                VALUES ($1, $2, $3)",
                &[
                    &favorite.base().id().reader_id().to_uuid()?,
                    &favorite.base().id().collection_id().to_uuid()?,
                    &favorite.base().created_at(),
                ],
            )
            .await
            .map_err(|err| Error::new("collection_favorite", "create").wrap_raw(err))?;

        Ok(())
    }

    async fn save_follow(&self, follow: &mut Follow) -> Result<()> {
        self.client
            .execute(
                "INSERT INTO follows(reader_id, author_id, datetime)
                VALUES ($1, $2, $3)",
                &[
                    &follow.base().id().reader_id().to_uuid()?,
                    &follow.base().id().author_id().to_uuid()?,
                    &follow.base().created_at(),
                ],
            )
            .await
            .map_err(|err| Error::new("collection_favorite", "create").wrap_raw(err))?;

        Ok(())
    }

    async fn delete_like(
        &self,
        reader_id: &ReaderId,
        publication_id: &PublicationId,
    ) -> Result<()> {
        self.client
            .execute(
                "DELETE FROM likes WHERE reader_id = $1 AND publication_id = $2",
                &[&reader_id.to_uuid()?, &publication_id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::new("like", "delete").wrap_raw(err))?;

        Ok(())
    }

    async fn delete_review(
        &self,
        reader_id: &ReaderId,
        publication_id: &PublicationId,
    ) -> Result<()> {
        self.client
            .execute(
                "DELETE FROM reviews WHERE reader_id = $1 AND publication_id = $2",
                &[&reader_id.to_uuid()?, &publication_id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::new("review", "delete").wrap_raw(err))?;

        Ok(())
    }

    async fn delete_publication_favorite(
        &self,
        reader_id: &ReaderId,
        publication_id: &PublicationId,
    ) -> Result<()> {
        self.client
            .execute(
                "DELETE FROM publication_favorites WHERE reader_id = $1 AND publication_id = $2",
                &[&reader_id.to_uuid()?, &publication_id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::new("publication_favorite", "delete").wrap_raw(err))?;

        Ok(())
    }

    async fn delete_collection_favorite(
        &self,
        reader_id: &ReaderId,
        collection_id: &CollectionId,
    ) -> Result<()> {
        self.client
            .execute(
                "DELETE FROM collection_favorites WHERE reader_id = $1 AND collection_id = $2",
                &[&reader_id.to_uuid()?, &collection_id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::new("collection_favorite", "delete").wrap_raw(err))?;

        Ok(())
    }

    async fn delete_follow(&self, reader_id: &ReaderId, author_id: &AuthorId) -> Result<()> {
        self.client
            .execute(
                "DELETE FROM collection_favorites WHERE reader_id = $1 AND author_id = $2",
                &[&reader_id.to_uuid()?, &author_id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::new("follow", "delete").wrap_raw(err))?;

        Ok(())
    }
}
