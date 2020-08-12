use serde::Serialize;

use common::result::Result;

use crate::application::util;
use crate::domain::user::{User, UserId, UserRepository};

#[derive(Serialize)]
pub struct GetByIdResponse {
    id: String,
    username: String,
    name: Option<String>,
    lastname: Option<String>,
}

impl From<&User> for GetByIdResponse {
    fn from(user: &User) -> GetByIdResponse {
        GetByIdResponse {
            id: user.base().id().value().to_owned(),
            username: user.identity().username().value().to_owned(),
            name: user.person().map(|p| p.fullname().name().to_owned()),
            lastname: user.person().map(|p| p.fullname().lastname().to_owned()),
        }
    }
}

pub struct GetById<'a, URepo> {
    user_repo: &'a URepo,
}

impl<'a, URepo> GetById<'a, URepo>
where
    URepo: UserRepository,
{
    pub fn new(user_repo: &'a URepo) -> Self {
        GetById { user_repo }
    }

    pub async fn exec(&self, auth_user: &User, user_id: &UserId) -> Result<GetByIdResponse> {
        util::is_authorized(auth_user, user_id)?;

        let user = self.user_repo.find_by_id(user_id).await?;
        Ok(GetByIdResponse::from(&user))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn not_owner() {
        let c = mocks::container();
        let uc = GetById::new(c.user_repo());

        let user = mocks::user1();
        let other_user = mocks::user2();
        assert!(uc.exec(&other_user, &user.base().id()).await.is_err());
    }

    #[tokio::test]
    async fn without_fullname() {
        let c = mocks::container();
        let uc = GetById::new(c.user_repo());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        let res = uc.exec(&user, &user.base().id()).await.unwrap();
        assert_eq!(res.username, user.identity().username().value());
        assert!(res.name.is_none());
        assert!(res.lastname.is_none());
    }

    #[tokio::test]
    async fn with_fullname() {
        let c = mocks::container();
        let uc = GetById::new(c.user_repo());

        let mut user = mocks::user1();
        user.set_person(mocks::person1()).unwrap();
        c.user_repo().save(&mut user).await.unwrap();

        let res = uc.exec(&user, &user.base().id()).await.unwrap();
        assert_eq!(res.id, user.base().id().value());
        assert_eq!(res.username, user.identity().username().value());
        assert_eq!(res.name.unwrap(), user.person().unwrap().fullname().name());
        assert_eq!(
            res.lastname.unwrap(),
            user.person().unwrap().fullname().lastname()
        );
    }
}
