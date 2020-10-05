use common::error::Error;
use common::result::Result;
use identity::domain::user::UserId;

use crate::application::dtos::SubscriptionDto;
use crate::domain::subscription::SubscriptionRepository;

pub struct GetByReader<'a> {
    subscription_repo: &'a dyn SubscriptionRepository,
}

impl<'a> GetByReader<'a> {
    pub fn new(subscription_repo: &'a dyn SubscriptionRepository) -> Self {
        GetByReader { subscription_repo }
    }

    pub async fn exec(&self, auth_id: String, reader_id: String) -> Result<SubscriptionDto> {
        if auth_id != reader_id {
            return Err(Error::unauthorized());
        }

        let subscription = self
            .subscription_repo
            .find_last_active_by_user_id(&UserId::new(reader_id)?)
            .await?;

        Ok(SubscriptionDto::from(&subscription))
    }
}
