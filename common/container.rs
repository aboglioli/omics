use async_trait::async_trait;

use crate::result::Result;

#[async_trait]
pub trait Container {
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
}
