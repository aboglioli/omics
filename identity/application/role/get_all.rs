use serde::Serialize;

use common::error::Error;
use common::result::Result;

use crate::application::dtos::RoleDto;
use crate::domain::role::RoleRepository;
use crate::domain::user::{UserId, UserRepository};

#[derive(Serialize)]
pub struct GetAllResponse {
    pub roles: Vec<RoleDto>,
}

pub struct GetAll<'a, RRepo, URepo> {
    role_repo: &'a RRepo,
    user_repo: &'a URepo,
}

impl<'a, RRepo, URepo> GetAll<'a, RRepo, URepo>
where
    RRepo: RoleRepository,
    URepo: UserRepository,
{
    pub fn new(role_repo: &'a RRepo, user_repo: &'a URepo) -> Self {
        GetAll {
            role_repo,
            user_repo,
        }
    }

    pub async fn exec(&self, admin_id: String) -> Result<GetAllResponse> {
        let admin = self.user_repo.find_by_id(&UserId::new(admin_id)?).await?;

        if !admin.role().is("admin") {
            return Err(Error::new("user", "unauthorized"));
        }

        let roles = self.role_repo.find_all().await?;

        Ok(GetAllResponse {
            roles: roles.iter().map(|role| RoleDto::new(role)).collect(),
        })
    }
}
