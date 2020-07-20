use common::error::Error;
use common::model::{Entity, ID};
use crate::domain::role::{Role, RoleID};
use crate::domain::user::{Email, Password, Person, Username};

// User
pub type UserID = String;

#[derive(Debug, Clone)]
pub struct User {
    id: ID<UserID>,
    username: Username,
    email: Email,
    password: Password,
    person: Option<Person>,
    role_id: RoleID,
    validated: bool,
}

impl User {
    pub fn new(
        id: UserID,
        username: &str,
        email: &str,
        password: &str,
        role: &Role,
    ) -> Result<User, Error> {
        let mut err = Error::application().set_code("user").build();

        let username = match Username::new(username) {
            Ok(username) => Some(username),
            Err(e) => {
                err.merge(e);
                None
            }
        };

        let email = match Email::new(email) {
            Ok(email) => Some(email),
            Err(e) => {
                err.merge(e);
                None
            }
        };

        let password = match Password::new(password) {
            Ok(password) => Some(password),
            Err(e) => {
                err.merge(e);
                None
            }
        };

        if err.has_context() {
            return Err(err.build());
        }

        Ok(User {
            id: ID::new(id),
            username: username.unwrap(),
            email: email.unwrap(),
            password: password.unwrap(),
            person: None,
            role_id: role.id().value(),
            validated: false,
        })
    }

    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password(&self) -> &Password {
        &self.password
    }

    pub fn person(&self) -> Option<&Person> {
        self.person.as_ref()
    }

    pub fn role(&self) -> &RoleID {
        &self.role_id
    }

    pub fn is_validated(&self) -> bool {
        self.validated
    }

    pub fn is_active(&self) -> bool {
        self.id.deleted_at().is_none() && self.validated
    }

    pub fn set_password(&mut self, hashed_password: &str) -> Result<(), Error> {
        self.password = Password::new(hashed_password)?;
        Ok(())
    }

    pub fn change_name(&mut self, name: &str, lastname: &str) -> Result<(), Error> {
        self.person = Some(Person::new(name, lastname)?);
        Ok(())
    }

    pub fn change_role(&mut self, role: &Role) {
        self.role_id = role.id().value();
    }
}

impl Entity<UserID> for User {
    fn id(&self) -> &ID<String> {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() -> Result<(), Error> {
        let role = Role::new(RoleID::from("user"), "User")?;
        let res = User::new(UserID::from(""), "", "", "", &role);
        assert!(res.is_err());
        assert_eq!(res.err().unwrap().context().len(), 3);

        Ok(())
    }
}
