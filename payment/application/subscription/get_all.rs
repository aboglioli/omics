use serde::Serialize;

use common::error::Error;
use common::request::Include;
use common::result::Result;
use identity::application::dtos::UserDto;
use identity::domain::user::{UserId, UserRepository};

use crate::application::dtos::SubscriptionDto;
use crate::domain::subscription::SubscriptionRepository;

#[derive(Serialize)]
pub struct GetAllResponse {
    subcriptions: Vec<SubscriptionDto>,
}

pub struct GetAll<'a> {
    subscription_repo: &'a dyn SubscriptionRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> GetAll<'a> {
    pub fn new(
        subscription_repo: &'a dyn SubscriptionRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        GetAll {
            subscription_repo,
            user_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, include: Include) -> Result<GetAllResponse> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.is_admin() {
            return Err(Error::unauthorized());
        }

        let subcriptions = self.subscription_repo.search(None, None, None).await?;

        let mut subscription_dtos = Vec::new();
        for subscription in subcriptions.iter() {
            let mut subscription_dto = SubscriptionDto::from(subscription);

            if include.has("user") {
                let user = self.user_repo.find_by_id(subscription.user_id()).await?;
                subscription_dto = subscription_dto.user(UserDto::from(&user));
            }

            subscription_dtos.push(subscription_dto);
        }

        Ok(GetAllResponse {
            subcriptions: subscription_dtos,
        })
    }
}
