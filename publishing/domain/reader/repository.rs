use async_trait::async_trait;

use common::result::Result;

use crate::domain::reader::{Reader, ReaderId};

#[async_trait]
pub trait ReaderRepository: Sync + Send {
    async fn next_id(&self) -> Result<ReaderId>;

    async fn find_by_id(&self, id: &ReaderId) -> Result<Reader>;

    async fn save(&self, reader: &mut Reader) -> Result<()>;
}
