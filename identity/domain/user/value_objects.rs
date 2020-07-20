use common::error::Error;

#[derive(Debug, Clone)]
pub struct Username {
    username: String,
}

impl Username {
    pub fn new(username: &str) -> Result<Username, Error> {
        if username.len() < 4 {
            return Err(Error::application()
                .add_context("username", "too_short")
                .clone());
        }

        if username.len() > 24 {
            return Err(Error::application()
                .add_context("username", "too_long")
                .clone());
        }

        Ok(Username {
            username: String::from(username),
        })
    }

    pub fn value(&self) -> &str {
        &self.username
    }
}

#[derive(Debug, Clone)]
pub struct Email {
    email: String,
}

impl Email {
    pub fn new(email: &str) -> Result<Email, Error> {
        if email.len() < 4 {
            return Err(Error::application()
                .add_context("email", "too_short")
                .clone());
        }

        if email.len() > 64 {
            return Err(Error::application()
                .add_context("email", "too_long")
                .clone());
        }

        Ok(Email {
            email: String::from(email),
        })
    }

    pub fn value(&self) -> &str {
        &self.email
    }
}

#[derive(Debug, Clone)]
pub struct Password {
    password: String,
}

impl Password {
    pub fn new(password: &str) -> Result<Password, Error> {
        if password.len() < 50 {
            return Err(Error::application()
                .add_context("password", "not_hashed")
                .clone());
        }

        Ok(Password {
            password: String::from(password),
        })
    }

    pub fn value(&self) -> &str {
        &self.password
    }
}

#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    lastname: String,
}

impl Person {
    pub fn new(name: &str, lastname: &str) -> Result<Person, Error> {
        let mut err = Error::application();
        if name.len() < 4 {
            err.add_context("name", "too_short");
        }

        if name.len() > 64 {
            err.add_context("name", "too_long");
        }

        if lastname.len() < 4 {
            err.add_context("lastname", "too_short");
        }

        if lastname.len() > 64 {
            err.add_context("lastname", "too_long");
        }

        if err.has_context() {
            return Err(err);
        }

        Ok(Person {
            name: String::from(name),
            lastname: String::from(lastname),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn lastname(&self) -> &str {
        &self.lastname
    }
}
