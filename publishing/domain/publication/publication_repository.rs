use async_trait::async_trait;

use common::error::Error;
use common::result::Result;

use crate::domain::publication::{Publication, PublicationId};

#[async_trait]
pub trait PublicationRepository {
    fn err_not_found() -> Error {
        Error::new("publication", "not_found")
    }

    async fn next_id(&self) -> Result<PublicationId>;

    async fn find_by_id(&self, id: &PublicationId) -> Result<Publication>;

    async fn save(&self, publication: &mut Publication) -> Result<()>;
}
