use async_trait::async_trait;
use uuid::Uuid;

use common::result::Result;

use crate::domain::reader::{Reader, ReaderId};

#[async_trait]
pub trait ReaderRepository: Sync + Send {
    async fn next_id(&self) -> Result<ReaderId> {
        ReaderId::new(Uuid::new_v4().to_string())
    }

    async fn find_by_id(&self, id: &ReaderId) -> Result<Reader>;

    async fn save(&self, reader: &mut Reader) -> Result<()>;
}
