use async_trait::async_trait;

use common::result::Result;

use crate::domain::admin::{Admin, AdminId};

#[async_trait]
pub trait AdminRepository: Sync + Send {
    async fn find_by_id(&self, id: &AdminId) -> Result<Admin>;

    async fn save(&self, admin: &mut Admin) -> Result<()>;
}
