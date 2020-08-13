use serde::Serialize;

use common::result::Result;

use crate::domain::user::{User, UserId, UserRepository};

#[derive(Serialize)]
pub struct GetByIdResponse {
    id: String,
    username: String,
    email: Option<String>,
    name: Option<String>,
    lastname: Option<String>,
}

impl From<&User> for GetByIdResponse {
    fn from(user: &User) -> GetByIdResponse {
        GetByIdResponse {
            id: user.base().id().value().to_owned(),
            username: user.identity().username().value().to_owned(),
            email: None,
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

    pub async fn exec(&self, viewer_id: String, user_id: String) -> Result<GetByIdResponse> {
        let user_id = UserId::new(user_id)?;
        let user = self.user_repo.find_by_id(&user_id).await?;

        let mut res = GetByIdResponse::from(&user);

        if viewer_id == user_id.value() {
            res.email = Some(user.identity().email().value().to_owned());
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn owner() {
        let c = mocks::container();
        let uc = GetById::new(c.user_repo());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        let id = user.base().id().value().to_owned();
        let res = uc.exec(id.clone(), id).await.unwrap();

        assert!(res.email.is_some());
    }

    #[tokio::test]
    async fn not_owner() {
        let c = mocks::container();
        let uc = GetById::new(c.user_repo());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        let id = user.base().id().value().to_owned();
        let res = uc
            .exec(mocks::user2().base().id().value().to_owned(), id)
            .await
            .unwrap();

        assert!(res.email.is_none());
    }

    #[tokio::test]
    async fn without_fullname() {
        let c = mocks::container();
        let uc = GetById::new(c.user_repo());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        let id = user.base().id().value().to_owned();
        let res = uc.exec(id.clone(), id).await.unwrap();
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

        let id = user.base().id().value().to_owned();
        let res = uc.exec(id.clone(), id).await.unwrap();
        assert_eq!(res.id, user.base().id().value());
        assert_eq!(res.username, user.identity().username().value());
        assert_eq!(res.name.unwrap(), user.person().unwrap().fullname().name());
        assert_eq!(
            res.lastname.unwrap(),
            user.person().unwrap().fullname().lastname()
        );
    }
}
