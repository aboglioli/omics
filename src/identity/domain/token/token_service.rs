use crate::common::cache::Cache;
use crate::common::error::Error;
use crate::identity::domain::token::{Data, Token, TokenEncoder, TokenID, TokenRepository};

pub struct TokenService<TE, TR> {
    token_encoder: TE,
    token_repository: TR,
}

impl<TE, TR> TokenService<TE, TR>
where
    TE: TokenEncoder,
    TR: TokenRepository,
{
    pub fn new(token_encoder: TE, token_repository: TR) -> TokenService<TE, TR> {
        TokenService {
            token_encoder,
            token_repository,
        }
    }

    pub fn create(&self, data: Data) -> Result<Token, Error> {
        let token_id = TokenID::new();
        self.token_repository.set(&token_id, data)?;
        let token = self.token_encoder.encode(&token_id)?;

        Ok(token)
    }

    pub fn validate(&self, token: Token) -> Result<&Data, Error> {
        let token_id = self.token_encoder.decode(token)?;
        if let Some(data) = self.token_repository.get(&token_id) {
            return Ok(data);
        }
        Err(Error::application().set_code("token_not_found").clone())
    }

    pub fn invalidate(&self, token: Token) -> Result<(), Error> {
        let token_id = self.token_encoder.decode(token)?;
        self.token_repository.delete(&token_id)?;
        Ok(())
    }
}
