use crate::domain::token::{Data, Token, TokenEncoder, TokenId, TokenRepository};
use common::error::Error;

pub struct TokenService<'a, TRepo, TEnc> {
    token_repository: &'a TRepo,
    token_encoder: &'a TEnc,
}

impl<'a, TRepo: TokenRepository, TEnc: TokenEncoder> TokenService<'a, TRepo, TEnc> {
    pub fn new(token_repository: &'a TRepo, token_encoder: &'a TEnc) -> Self {
        TokenService {
            token_encoder,
            token_repository,
        }
    }

    pub async fn create(&self, data: Data) -> Result<Token, Error> {
        let token_id = TokenId::new();
        let token = self.token_encoder.encode(&token_id)?;
        self.token_repository.set(token_id, data).await?;

        Ok(token)
    }

    pub async fn validate(&self, token: &Token) -> Result<Data, Error> {
        let token_id = self.token_encoder.decode(token)?;
        if let Some(data) = self.token_repository.get(&token_id).await {
            return Ok(data);
        }
        Err(Error::application().set_code("token_not_found").build())
    }

    pub async fn invalidate(&self, token: &Token) -> Result<(), Error> {
        let token_id = self.token_encoder.decode(token)?;
        self.token_repository.delete(&token_id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::infrastructure::mocks::FakeTokenEncoder;
    use crate::infrastructure::persistence::inmem::InMemTokenRepository;

    #[tokio::test]
    async fn create() -> Result<(), Error> {
        let enc = FakeTokenEncoder::new();
        let repo = InMemTokenRepository::new();
        let serv = TokenService::new(&repo, &enc);

        let mut data = Data::new();
        data.add("user_id", "u123");
        data.add("user_username", "admin");

        let token = serv.create(data).await?;
        assert!(!token.token().is_empty());

        let data = serv.validate(&token).await?;
        assert_eq!(data.get("user_id"), Some(&"u123".to_owned()));
        assert_eq!(data.get("user_username"), Some(&"admin".to_owned()));

        assert!(serv.invalidate(&token).await.is_ok());

        assert!(serv.validate(&token).await.is_err());

        Ok(())
    }
}
