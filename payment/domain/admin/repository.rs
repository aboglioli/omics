use async_trait::async_trait;
use uuid::Uuid;

use common::result::Result;

use crate::domain::admin::{Admin, AdminId};

#[async_trait]
pub trait AdminRepository: Sync + Send {
    async fn next_id(&self) -> Result<AdminId> {
        AdminId::new(Uuid::new_v4().to_string())
    }

    async fn find_by_id(&self, id: &AdminId) -> Result<Admin>;

    async fn save(&self, admin: &mut Admin) -> Result<()>;
}
