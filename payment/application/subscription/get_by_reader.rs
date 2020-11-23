use common::error::Error;
use common::result::Result;
use identity::domain::user::UserId;
use identity::UserIdAndRole;

use crate::application::dtos::SubscriptionDto;
use crate::domain::subscription::SubscriptionRepository;

pub struct GetByReader<'a> {
    subscription_repo: &'a dyn SubscriptionRepository,
}

impl<'a> GetByReader<'a> {
    pub fn new(subscription_repo: &'a dyn SubscriptionRepository) -> Self {
        GetByReader { subscription_repo }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        reader_id: String,
    ) -> Result<SubscriptionDto> {
        if auth_id.value() != reader_id || !auth_role.can("subscribe") {
            return Err(Error::unauthorized());
        }

        let subscription = self
            .subscription_repo
            .find_by_user_id(&UserId::new(reader_id)?)
            .await?;

        Ok(SubscriptionDto::from(&subscription))
    }
}
