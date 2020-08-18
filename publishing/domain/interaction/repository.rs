use async_trait::async_trait;
use chrono::{DateTime, Utc};

use common::result::Result;

use crate::domain::interaction::{Like, Reading, Review, View};
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

#[async_trait]
pub trait InteractionRepository: Sync + Send {
    async fn find_views(
        &self,
        reader_id: Option<&ReaderId>,
        publication_id: Option<&PublicationId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<View>>;
    async fn find_readings(
        &self,
        reader_id: Option<&ReaderId>,
        publication_id: Option<&PublicationId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Reading>>;
    async fn find_likes(
        &self,
        reader_id: Option<&ReaderId>,
        publication_id: Option<&PublicationId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Like>>;
    async fn find_reviews(
        &self,
        reader_id: Option<&ReaderId>,
        publication_id: Option<&PublicationId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Review>>;

    async fn save_view(&self, view: &mut View) -> Result<()>;
    async fn save_reading(&self, reading: &mut Reading) -> Result<()>;
    async fn save_like(&self, like: &mut Like) -> Result<()>;
    async fn save_review(&self, review: &mut Review) -> Result<()>;

    async fn delete_like(&self, reader_id: &ReaderId, publication_id: &PublicationId)
        -> Result<()>;
    async fn delete_review(
        &self,
        reader_id: &ReaderId,
        publication_id: &PublicationId,
    ) -> Result<()>;
}
