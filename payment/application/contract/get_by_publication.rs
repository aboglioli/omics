use common::error::Error;
use common::result::Result;
use identity::UserIdAndRole;
use publishing::domain::publication::{PublicationId, PublicationRepository};

use crate::application::dtos::ContractDto;
use crate::domain::contract::ContractRepository;

pub struct GetByPublication<'a> {
    contract_repo: &'a dyn ContractRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> GetByPublication<'a> {
    pub fn new(
        contract_repo: &'a dyn ContractRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        GetByPublication {
            contract_repo,
            publication_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        publication_id: String,
    ) -> Result<ContractDto> {
        let publication = self
            .publication_repo
            .find_by_id(&PublicationId::new(publication_id)?)
            .await?;
        if publication.author_id() != &auth_id && !auth_role.can("get_contract_by_publication") {
            return Err(Error::unauthorized());
        }

        let contract = self
            .contract_repo
            .find_by_publication_id(publication.base().id())
            .await?;

        Ok(ContractDto::from(&contract))
    }
}
