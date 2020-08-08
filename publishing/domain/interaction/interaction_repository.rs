use async_trait::async_trait;

use common::result::Result;

use crate::domain::interaction::{Like, Reading, Review, View};
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

pub struct FindOpts {
    pub publication_id: Option<PublicationId>,
    pub reader_id: Option<ReaderId>,
}

#[async_trait]
pub trait InteractionRepository {
    async fn find_views(&self, opts: &FindOpts) -> Result<Vec<View>>;
    async fn find_likes(&self, opts: &FindOpts) -> Result<Vec<Like>>;
    async fn find_readings(&self, opts: &FindOpts) -> Result<Vec<Reading>>;
    async fn find_reviews(&self, opts: &FindOpts) -> Result<Vec<Review>>;

    async fn save_view(&self, view: &mut View) -> Result<()>;
    async fn save_like(&self, like: &mut Like) -> Result<()>;
    async fn save_reading(&self, read: &mut Reading) -> Result<()>;
    async fn save_review(&self, review: &mut Review) -> Result<()>;
}
