use crate::common::error::Error;
use crate::identity::domain::token::{Data, Token, TokenEncoder, TokenID, TokenRepository};

pub struct TokenService<TTokenEncoder, TTokenRepository> {
    token_encoder: TTokenEncoder,
    token_repository: TTokenRepository,
}

impl<TTokenEncoder, TTokenRepository> TokenService<TTokenEncoder, TTokenRepository>
where
    TTokenEncoder: TokenEncoder,
    TTokenRepository: TokenRepository,
{
    pub fn new(
        token_encoder: TTokenEncoder,
        token_repository: TTokenRepository,
    ) -> TokenService<TTokenEncoder, TTokenRepository> {
        TokenService {
            token_encoder,
            token_repository,
        }
    }

    pub fn create(&self, data: Data) -> Result<Token, Error> {
        let token_id = TokenID::new();
        let token = self.token_encoder.encode(&token_id)?;
        self.token_repository.set(token_id, data)?;

        Ok(token)
    }

    pub fn validate(&self, token: Token) -> Result<Data, Error> {
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

    pub fn token_repository(&self) -> &TTokenRepository {
        &self.token_repository
    }
}
