use async_trait::async_trait;

use common::error::Error;

use crate::domain::publication::{Publication, PublicationId};

#[async_trait]
pub trait PublicationRepository {
    async fn find_by_id(&self, id: &PublicationId) -> Result<Publication, Error>;
}
