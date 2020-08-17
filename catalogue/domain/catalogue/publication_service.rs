use async_trait::async_trait;

use common::result::Result;

use crate::domain::catalogue::Publication;

#[async_trait]
pub trait PublicationService {
    async fn get_by_id(&self, id: &str) -> Result<Publication>;
}
