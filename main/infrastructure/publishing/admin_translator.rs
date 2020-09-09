use std::sync::Arc;

use async_trait::async_trait;

use common::error::Error;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use publishing::domain::admin::{Admin, AdminId, AdminRepository};

pub struct AdminTranslator {
    user_repo: Arc<dyn UserRepository>,
}

impl AdminTranslator {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        AdminTranslator { user_repo }
    }
}

#[async_trait]
impl AdminRepository for AdminTranslator {
    async fn find_by_id(&self, id: &AdminId) -> Result<Admin> {
        let user = self.user_repo.find_by_id(&UserId::new(id.value())?).await?;

        if !user.role().is("admin") {
            return Err(Error::unauthorized());
        }

        Ok(Admin::new(user.base().id().clone())?)
    }

    async fn save(&self, _admin: &mut Admin) -> Result<()> {
        Ok(())
    }
}
