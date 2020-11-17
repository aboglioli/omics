use common::error::Error;
use common::request::Include;
use common::result::Result;

use crate::application::dtos::{RoleDto, UserDto};
use crate::domain::role::RoleRepository;
use crate::domain::user::{UserId, UserRepository};
use crate::UserIdAndRole;

pub struct GetById<'a> {
    role_repo: &'a dyn RoleRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> GetById<'a> {
    pub fn new(role_repo: &'a dyn RoleRepository, user_repo: &'a dyn UserRepository) -> Self {
        GetById {
            role_repo,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        user_id: String,
        include: Include,
    ) -> Result<UserDto> {
        let user_id = UserId::new(user_id)?;

        if !auth_role.can("get_all_users") {
            if auth_id != user_id {
                return Err(Error::unauthorized());
            } else if !auth_role.can("get_own_user") {
                return Err(Error::unauthorized());
            }
        }

        let user = self.user_repo.find_by_id(&user_id).await?;
        let mut user_dto = UserDto::from(&user);

        if include.has("role") {
            let role = self.role_repo.find_by_id(user.role_id()).await?;
            user_dto = user_dto.role(RoleDto::from(&role));
        }

        Ok(user_dto)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn owner() {
        let c = mocks::container();
        let uc = GetById::new(c.role_repo(), c.user_repo());

        let mut user = mocks::user(
            "user-1",
            "username",
            "user@omics.com",
            "P@asswd!",
            true,
            None,
            None,
            "user",
        );
        c.user_repo().save(&mut user).await.unwrap();
        let role = mocks::role("User");

        let id = user.base().id().to_string();
        let res = uc
            .exec((user.base().id().clone(), role), id, Include::default())
            .await
            .unwrap();

        assert_eq!(res.id, user.base().id().value());
    }

    #[tokio::test]
    async fn not_owner() {
        let c = mocks::container();
        let uc = GetById::new(c.role_repo(), c.user_repo());

        let mut user = mocks::user(
            "user-1",
            "username",
            "user@omics.com",
            "P@asswd!",
            true,
            None,
            None,
            "user",
        );
        c.user_repo().save(&mut user).await.unwrap();
        let role = mocks::role("User");

        let id = user.base().id().to_string();
        assert!(uc
            .exec(
                (UserId::new("user-2").unwrap(), role),
                id,
                Include::default()
            )
            .await
            .is_err());
    }

    #[tokio::test]
    async fn admin_not_owner() {
        let c = mocks::container();
        let uc = GetById::new(c.role_repo(), c.user_repo());

        let mut user = mocks::user(
            "user-1",
            "username",
            "user@omics.com",
            "P@asswd!",
            true,
            None,
            None,
            "user",
        );
        c.user_repo().save(&mut user).await.unwrap();
        let role = mocks::role("User");

        let mut admin = mocks::user(
            "admin-1",
            "admin",
            "admin@omics.com",
            "P@asswd!",
            true,
            None,
            None,
            "admin",
        );
        c.user_repo().save(&mut admin).await.unwrap();

        assert!(uc
            .exec(
                (admin.base().id().clone(), role),
                user.base().id().to_string(),
                Include::default()
            )
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn without_fullname() {
        let c = mocks::container();
        let uc = GetById::new(c.role_repo(), c.user_repo());

        let mut user = mocks::user(
            "user-1",
            "username",
            "user@omics.com",
            "P@asswd!",
            true,
            None,
            None,
            "user",
        );
        c.user_repo().save(&mut user).await.unwrap();
        let role = mocks::role("User");

        let id = user.base().id().to_string();
        let res = uc
            .exec((user.base().id().clone(), role), id, Include::default())
            .await
            .unwrap();
        assert_eq!(res.username, user.identity().username().value());
        assert!(res.name.is_none());
        assert!(res.lastname.is_none());
    }

    #[tokio::test]
    async fn with_fullname() {
        let c = mocks::container();
        let uc = GetById::new(c.role_repo(), c.user_repo());

        let mut user = mocks::user(
            "user-1",
            "username",
            "user@omics.com",
            "P@asswd!",
            true,
            Some("Name"),
            Some("Lastname"),
            "user",
        );
        c.user_repo().save(&mut user).await.unwrap();
        let role = mocks::role("User");

        let id = user.base().id().to_string();
        let res = uc
            .exec((user.base().id().clone(), role), id, Include::default())
            .await
            .unwrap();
        assert_eq!(res.id, user.base().id().value());
        assert_eq!(res.username, user.identity().username().value());
        assert_eq!(res.name.unwrap(), user.person().unwrap().fullname().name());
        assert_eq!(
            res.lastname.unwrap(),
            user.person().unwrap().fullname().lastname()
        );
    }
}
