use std::str::FromStr;

use chrono::DateTime;
use serde::Deserialize;

use common::error::Error;
use common::request::Include;
use common::request::{PaginationParams, PaginationResponse};
use common::result::Result;

use crate::application::dtos::{RoleDto, UserDto};
use crate::domain::role::{RoleId, RoleRepository};
use crate::domain::user::{UserOrderBy, UserRepository};
use crate::UserIdAndRole;

#[derive(Deserialize)]
pub struct SearchCommand {
    pub role_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

pub struct Search<'a> {
    role_repo: &'a dyn RoleRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Search<'a> {
    pub fn new(role_repo: &'a dyn RoleRepository, user_repo: &'a dyn UserRepository) -> Self {
        Search {
            role_repo,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        (_auth_id, auth_role): UserIdAndRole,
        cmd: SearchCommand,
        include: Include,
        pagination: PaginationParams,
    ) -> Result<PaginationResponse<UserDto>> {
        if !auth_role.can("get_any_user") {
            return Err(Error::unauthorized());
        }

        let pagination_users = self
            .user_repo
            .search(
                cmd.role_id.map(RoleId::new).transpose()?.as_ref(),
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
                    .map(|o| UserOrderBy::from_str(&o))
                    .transpose()?
                    .as_ref(),
            )
            .await?;

        let mut res = PaginationResponse::new(
            pagination_users.offset(),
            pagination_users.limit(),
            pagination_users.total(),
            pagination_users.matching_criteria(),
        );

        for user in pagination_users.into_items().into_iter() {
            let mut user_dto = UserDto::from(&user);

            if include.has("role") {
                let role = self.role_repo.find_by_id(user.role_id()).await?;
                user_dto = user_dto.role(RoleDto::from(&role));
            }

            res.add_item(user_dto);
        }

        Ok(res)
    }
}
