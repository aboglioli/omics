use std::str::FromStr;

use chrono::DateTime;
use serde::Deserialize;

use common::error::Error;
use common::result::Result;
use identity::UserIdAndRole;

use crate::application::dtos::StatisticsDto;
use crate::domain::publication::{PublicationId, PublicationRepository, StatisticsService};

#[derive(Deserialize)]
pub struct GetStatisticsCommand {
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

pub struct GetStatistics<'a> {
    publication_repo: &'a dyn PublicationRepository,

    statistics_serv: &'a StatisticsService,
}

impl<'a> GetStatistics<'a> {
    pub fn new(
        publication_repo: &'a dyn PublicationRepository,
        statistics_serv: &'a StatisticsService,
    ) -> Self {
        GetStatistics {
            publication_repo,
            statistics_serv,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        publication_id: String,
        cmd: GetStatisticsCommand,
    ) -> Result<StatisticsDto> {
        let publication = self
            .publication_repo
            .find_by_id(&PublicationId::new(publication_id)?)
            .await?;

        if !auth_role.can("get_any_publication") {
            if publication.author_id() != &auth_id || !auth_role.can("get_own_publication") {
                return Err(Error::unauthorized());
            }
        }

        let statistics = self
            .statistics_serv
            .get_history(
                None,
                Some(publication.base().id()),
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
            )
            .await?;

        Ok(StatisticsDto::from(&statistics))
    }
}
