use common::error::Error;
use common::result::Result;

use crate::domain::token::{Data, Token, TokenEncoder, TokenId, TokenRepository};

pub struct TokenService<'a, TRepo, TEnc> {
    token_repo: &'a TRepo,

    token_enc: &'a TEnc,
}

impl<'a, TRepo: TokenRepository, TEnc: TokenEncoder> TokenService<'a, TRepo, TEnc> {
    pub fn new(token_repo: &'a TRepo, token_enc: &'a TEnc) -> Self {
        TokenService {
            token_enc,
            token_repo,
        }
    }

    pub async fn create(&self, data: Data) -> Result<Token> {
        let token_id = TokenId::new();
        let token = self.token_enc.encode(&token_id)?;
        self.token_repo.set(token_id, data).await?;

        Ok(token)
    }

    pub async fn validate(&self, token: &Token) -> Result<Data> {
        let token_id = self.token_enc.decode(token)?;
        if let Some(data) = self.token_repo.get(&token_id).await {
            return Ok(data);
        }
        Err(Error::new("token", "not_found"))
    }

    pub async fn invalidate(&self, token: &Token) -> Result<()> {
        let token_id = self.token_enc.decode(token)?;
        self.token_repo.delete(&token_id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::infrastructure::persistence::inmem::InMemTokenRepository;
    use crate::mocks::FakeTokenEncoder;

    #[tokio::test]
    async fn create_validate_invalidate() {
        let repo = InMemTokenRepository::new();
        let enc = FakeTokenEncoder::new();
        let serv = TokenService::new(&repo, &enc);

        let mut data = Data::new();
        data.add("user_id", "u123");
        data.add("user_username", "admin");

        let token = serv.create(data).await.unwrap();
        assert!(!token.value().is_empty());

        let data = serv.validate(&token).await.unwrap();
        assert_eq!(data.get("user_id"), Some(&"u123".to_owned()));
        assert_eq!(data.get("user_username"), Some(&"admin".to_owned()));

        assert!(serv.invalidate(&token).await.is_ok());

        assert!(serv.validate(&token).await.is_err());
    }
}
