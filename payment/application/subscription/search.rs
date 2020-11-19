use std::str::FromStr;

use chrono::DateTime;
use serde::Deserialize;

use common::error::Error;
use common::request::{Include, PaginationParams, PaginationResponse};
use common::result::Result;
use identity::application::dtos::UserDto;
use identity::domain::user::{UserId, UserRepository};
use identity::UserIdAndRole;

use crate::application::dtos::SubscriptionDto;
use crate::domain::plan::PlanId;
use crate::domain::subscription::{Status, SubscriptionOrderBy, SubscriptionRepository};

#[derive(Deserialize)]
pub struct SearchCommand {
    pub user_id: Option<String>,
    pub plan_id: Option<String>,
    pub status: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

pub struct Search<'a> {
    subscription_repo: &'a dyn SubscriptionRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Search<'a> {
    pub fn new(
        subscription_repo: &'a dyn SubscriptionRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        Search {
            subscription_repo,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        cmd: SearchCommand,
        include: Include,
        pagination: PaginationParams,
    ) -> Result<PaginationResponse<SubscriptionDto>> {
        if !auth_role.can("search_subscriptions") {
            return Err(Error::unauthorized());
        }

        let pagination_subscriptions = self
            .subscription_repo
            .search(
                cmd.user_id.map(UserId::new).transpose()?.as_ref(),
                cmd.plan_id.map(PlanId::new).transpose()?.as_ref(),
                cmd.status
                    .map(|s| Status::from_str(&s))
                    .transpose()?
                    .as_ref(),
                cmd.date_from
                    .map(|d| DateTime::from_str(&d))
                    .transpose()
                    .map_err(|err| Error::bad_format("date_from").wrap_raw(err))?
                    .as_ref(),
                cmd.date_to
                    .map(|d| DateTime::from_str(&d))
                    .transpose()
                    .map_err(|err| Error::bad_format("date_to").wrap_raw(err))?
                    .as_ref(),
                pagination.offset(),
                pagination.limit(),
                pagination
                    .order_by()
                    .map(|o| SubscriptionOrderBy::from_str(&o))
                    .transpose()?
                    .as_ref(),
            )
            .await?;

        let mut res = PaginationResponse::from(&pagination_subscriptions);

        for subscription in pagination_subscriptions.into_items().into_iter() {
            let mut subscription_dto = SubscriptionDto::from(&subscription);

            if include.has("user") {
                let user = self.user_repo.find_by_id(subscription.user_id()).await?;
                subscription_dto = subscription_dto.user(UserDto::from(&user));
            }

            res.add_item(subscription_dto);
        }

        Ok(res)
    }
}
