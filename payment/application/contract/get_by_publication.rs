use common::error::Error;
use common::result::Result;
use identity::domain::user::UserRepository;
use identity::UserIdAndRole;
use publishing::domain::publication::{PublicationId, PublicationRepository};

use crate::application::dtos::ContractDto;
use crate::domain::contract::ContractRepository;

pub struct GetByPublication<'a> {
    contract_repo: &'a dyn ContractRepository,
    publication_repo: &'a dyn PublicationRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> GetByPublication<'a> {
    pub fn new(
        contract_repo: &'a dyn ContractRepository,
        publication_repo: &'a dyn PublicationRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        GetByPublication {
            contract_repo,
            publication_repo,
            user_repo,
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
