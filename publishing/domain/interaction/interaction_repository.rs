use async_trait::async_trait;

use common::error::Error;

use crate::domain::interaction::{Like, Read, Review};
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

pub struct FindOpts {
    pub publication_id: Option<PublicationId>,
    pub reader_id: Option<ReaderId>,
}

#[async_trait]
pub trait InteractionRepository {
    async fn find_likes(&self, opts: &FindOpts) -> Result<Vec<Like>, Error>;
    async fn find_reads(&self, opts: &FindOpts) -> Result<Vec<Read>, Error>;
    async fn find_reviews(&self, opts: &FindOpts) -> Result<Vec<Review>, Error>;

    async fn save_like(&self, like: &mut Like) -> Result<(), Error>;
    async fn save_read(&self, read: &mut Read) -> Result<(), Error>;
    async fn save_review(&self, review: &mut Review) -> Result<(), Error>;
}
