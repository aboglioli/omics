use async_trait::async_trait;

use common::result::Result;

use crate::domain::reader::{Reader, ReaderId};

#[async_trait]
pub trait ReaderRepository {
    async fn find_by_id(&self, id: &ReaderId) -> Result<Reader>;
}
