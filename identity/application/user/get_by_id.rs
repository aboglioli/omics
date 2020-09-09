use common::error::Error;
use common::request::Include;
use common::result::Result;

use crate::application::dtos::{RoleDto, UserDto};
use crate::domain::role::RoleRepository;
use crate::domain::user::{UserId, UserRepository};

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
        auth_id: String,
        user_id: String,
        include: Include,
    ) -> Result<UserDto> {
        if auth_id != user_id {
            let auth_user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
            if auth_user.role_id().value() != "admin" {
                return Err(Error::unauthorized());
            }
        }

        let user = self.user_repo.find_by_id(&UserId::new(user_id)?).await?;
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

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        let id = user.base().id().to_string();
        let res = uc.exec(id.clone(), id, Include::default()).await.unwrap();

        assert_eq!(res.id, user.base().id().value());
    }

    #[tokio::test]
    async fn not_owner() {
        let c = mocks::container();
        let uc = GetById::new(c.role_repo(), c.user_repo());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        let id = user.base().id().to_string();
        assert!(uc
            .exec(
                mocks::user2().base().id().to_string(),
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

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();
        let mut admin = mocks::admin1();
        c.user_repo().save(&mut admin).await.unwrap();

        assert!(uc
            .exec(
                admin.base().id().to_string(),
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

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        let id = user.base().id().to_string();
        let res = uc.exec(id.clone(), id, Include::default()).await.unwrap();
        assert_eq!(res.username, user.identity().username().value());
        assert!(res.name.is_none());
        assert!(res.lastname.is_none());
    }

    #[tokio::test]
    async fn with_fullname() {
        let c = mocks::container();
        let uc = GetById::new(c.role_repo(), c.user_repo());

        let mut user = mocks::user1();
        user.set_person(mocks::person1()).unwrap();
        c.user_repo().save(&mut user).await.unwrap();

        let id = user.base().id().to_string();
        let res = uc.exec(id.clone(), id, Include::default()).await.unwrap();
        assert_eq!(res.id, user.base().id().value());
        assert_eq!(res.username, user.identity().username().value());
        assert_eq!(res.name.unwrap(), user.person().unwrap().fullname().name());
        assert_eq!(
            res.lastname.unwrap(),
            user.person().unwrap().fullname().lastname()
        );
    }
}
