use crate::common::error::Error;
use crate::identity::domain::token::{Data, Token, TokenEncoder, TokenID, TokenRepository};

pub trait TokenService {
    fn create(&self, data: Data) -> Result<Token, Error>;
    fn validate(&self, token: Token) -> Result<Data, Error>;
    fn invalidate(&self, token: Token) -> Result<(), Error>;
}

pub struct TokenServiceImpl<TTokenEncoder, TTokenRepository> {
    token_encoder: TTokenEncoder,
    token_repository: TTokenRepository,
}

impl<TTokenEncoder, TTokenRepository> TokenServiceImpl<TTokenEncoder, TTokenRepository>
where
    TTokenEncoder: TokenEncoder,
    TTokenRepository: TokenRepository,
{
    pub fn new(
        token_encoder: TTokenEncoder,
        token_repository: TTokenRepository,
    ) -> Self {
        TokenServiceImpl {
            token_encoder,
            token_repository,
        }
    }
}

impl<TTokenEncoder, TTokenRepository> TokenService
    for TokenServiceImpl<TTokenEncoder, TTokenRepository>
where
    TTokenEncoder: TokenEncoder,
    TTokenRepository: TokenRepository,
{
    fn create(&self, data: Data) -> Result<Token, Error> {
        let token_id = TokenID::new();
        let token = self.token_encoder.encode(&token_id)?;
        self.token_repository.set(token_id, data)?;

        Ok(token)
    }

    fn validate(&self, token: Token) -> Result<Data, Error> {
        let token_id = self.token_encoder.decode(token)?;
        if let Some(data) = self.token_repository.get(&token_id) {
            return Ok(data);
        }
        Err(Error::application().set_code("token_not_found").build())
    }

    fn invalidate(&self, token: Token) -> Result<(), Error> {
        let token_id = self.token_encoder.decode(token)?;
        self.token_repository.delete(&token_id)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::identity::infrastructure::mocks::FakeTokenEncoder;
    use crate::identity::infrastructure::persistence::inmem::InMemTokenRepository;

    #[test]
    fn create() -> Result<(), Error> {
        let enc = FakeTokenEncoder::new();
        let repo = InMemTokenRepository::new();
        let serv = TokenServiceImpl::new(enc, repo);

        let data = Data::new();
        data.add("user_id", "u123");
        data.add("user_username", "admin");

        let token = serv.create(data)?;

        assert!(token.token().len() > 0);

        Ok(())
    }
}
