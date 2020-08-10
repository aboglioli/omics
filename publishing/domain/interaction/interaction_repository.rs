use async_trait::async_trait;
use chrono::{DateTime, Utc};

use common::result::Result;

use crate::domain::interaction::{Like, Reading, Review, View};
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

pub struct FindOpts<'a> {
    pub publication_id: Option<&'a PublicationId>,
    pub reader_id: Option<&'a ReaderId>,
    pub from: Option<&'a DateTime<Utc>>,
    pub to: Option<&'a DateTime<Utc>>,
}

#[async_trait]
pub trait InteractionRepository {
    async fn find_views(&self, opts: &FindOpts<'_>) -> Result<Vec<View>>;
    async fn find_readings(&self, opts: &FindOpts<'_>) -> Result<Vec<Reading>>;
    async fn find_likes(&self, opts: &FindOpts<'_>) -> Result<Vec<Like>>;
    async fn find_reviews(&self, opts: &FindOpts<'_>) -> Result<Vec<Review>>;

    async fn save_view(&self, view: &mut View) -> Result<()>;
    async fn save_reading(&self, read: &mut Reading) -> Result<()>;
    async fn save_like(&self, like: &mut Like) -> Result<()>;
    async fn save_review(&self, review: &mut Review) -> Result<()>;
}
