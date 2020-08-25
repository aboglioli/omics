use serde::Serialize;

use crate::domain::contract::Contract;
use crate::domain::publication::Publication;
use crate::domain::user::User;

#[derive(Serialize)]
pub struct UserDto {
    id: String,
}

impl From<&User> for UserDto {
    fn from(user: &User) -> Self {
        UserDto {
            id: user.base().id().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct PublicationDto {
    id: String,
    user: UserDto,
}

impl From<&Publication> for PublicationDto {
    fn from(publication: &Publication) -> Self {
        PublicationDto {
            id: publication.base().id().to_string(),
            user: UserDto::from(publication.author()),
        }
    }
}

#[derive(Serialize)]
pub struct ContractDto {
    pub id: String,
    pub publication: PublicationDto,
}

impl From<&Contract> for ContractDto {
    fn from(contract: &Contract) -> Self {
        ContractDto {
            id: contract.base().id().to_string(),
            publication: PublicationDto::from(contract.publication()),
        }
    }
}
