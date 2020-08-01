use async_trait::async_trait;

use common::result::Result;

use crate::domain::interaction::{Like, Read, Review};
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

pub struct FindOpts {
    pub publication_id: Option<PublicationId>,
    pub reader_id: Option<ReaderId>,
}

#[async_trait]
pub trait InteractionRepository {
    async fn find_likes(&self, opts: &FindOpts) -> Result<Vec<Like>>;
    async fn find_reads(&self, opts: &FindOpts) -> Result<Vec<Read>>;
    async fn find_reviews(&self, opts: &FindOpts) -> Result<Vec<Review>>;

    async fn save_like(&self, like: &mut Like) -> Result<()>;
    async fn save_read(&self, read: &mut Read) -> Result<()>;
    async fn save_review(&self, review: &mut Review) -> Result<()>;
}
