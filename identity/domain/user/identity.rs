use crate::domain::user::{Email, Password, Provider, Username};
use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Identity {
    provider: Provider,
    username: Username,
    email: Email,
    password: Option<Password>,
}

impl Identity {
    pub fn new(
        provider: Provider,
        username: Username,
        email: Email,
        password: Option<Password>,
    ) -> Result<Identity> {
        let password = match provider {
            Provider::Local => match password {
                None => return Err(Error::pair("password", "required")),
                password => password,
            },
            _ => None,
        };

        Ok(Identity {
            provider,
            username,
            email,
            password,
        })
    }

    pub fn provider(&self) -> &Provider {
        &self.provider
    }

    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password(&self) -> Option<&Password> {
        self.password.as_ref()
    }

    pub fn set_password(&mut self, password: Password) -> Result<()> {
        self.password = match self.provider {
            Provider::Local => Some(password),
            _ => return Err(Error::application()),
        };
        Ok(())
    }
}
