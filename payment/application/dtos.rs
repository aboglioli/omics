use serde::Serialize;

use crate::domain::contract::Contract;
use crate::domain::publication::Publication;
use crate::domain::user::User;

#[derive(Serialize)]
pub struct UserDto {
    id: String,
}

impl UserDto {
    pub fn new(user: &User) -> Self {
        UserDto {
            id: user.id().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct PublicationDto {
    id: String,
    user: UserDto,
}

impl PublicationDto {
    pub fn new(publication: &Publication) -> Self {
        PublicationDto {
            id: publication.id().to_string(),
            user: UserDto::new(publication.author()),
        }
    }
}

#[derive(Serialize)]
pub struct ContractDto {
    pub id: String,
    pub publication: PublicationDto,
}

impl ContractDto {
    pub fn new(contract: &Contract) -> Self {
        ContractDto {
            id: contract.base().id().to_string(),
            publication: PublicationDto::new(contract.publication()),
        }
    }
}
