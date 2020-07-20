use std::rc::Rc;

use crate::domain::token::{Data, Token, TokenEncoder, TokenID, TokenRepository};
use common::error::Error;

pub trait TokenService {
    fn create(&self, data: Data) -> Result<Token, Error>;
    fn validate(&self, token: &Token) -> Result<Data, Error>;
    fn invalidate(&self, token: &Token) -> Result<(), Error>;
}

pub struct TokenServiceImpl<TTokenEncoder, TTokenRepository> {
    token_encoder: Rc<TTokenEncoder>,
    token_repository: Rc<TTokenRepository>,
}

impl<TTokenEncoder, TTokenRepository> TokenServiceImpl<TTokenEncoder, TTokenRepository>
where
    TTokenEncoder: TokenEncoder,
    TTokenRepository: TokenRepository,
{
    pub fn new(token_encoder: Rc<TTokenEncoder>, token_repository: Rc<TTokenRepository>) -> Self {
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

    fn validate(&self, token: &Token) -> Result<Data, Error> {
        let token_id = self.token_encoder.decode(token)?;
        if let Some(data) = self.token_repository.get(&token_id) {
            return Ok(data);
        }
        Err(Error::application().set_code("token_not_found").build())
    }

    fn invalidate(&self, token: &Token) -> Result<(), Error> {
        let token_id = self.token_encoder.decode(token)?;
        self.token_repository.delete(&token_id)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::infrastructure::mocks::FakeTokenEncoder;
    use crate::infrastructure::persistence::inmem::InMemTokenRepository;

    #[test]
    fn create() -> Result<(), Error> {
        let enc = Rc::new(FakeTokenEncoder::new());
        let repo = Rc::new(InMemTokenRepository::new());
        let serv = TokenServiceImpl::new(Rc::clone(&enc), Rc::clone(&repo));

        let mut data = Data::new();
        data.add("user_id", "u123");
        data.add("user_username", "admin");

        let token = serv.create(data)?;
        assert!(!token.token().is_empty());

        let data = serv.validate(&token)?;
        assert_eq!(data.get("user_id"), Some(&"u123".to_owned()));
        assert_eq!(data.get("user_username"), Some(&"admin".to_owned()));

        assert!(serv.invalidate(&token).is_ok());

        assert!(serv.validate(&token).is_err());

        Ok(())
    }
}
