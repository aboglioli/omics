use crate::common::error::Error;
use crate::common::model::{Entity, ID};
use crate::identity::domain::role::{Role, RoleID};
use crate::identity::domain::user::{Email, Password, Person, Username};

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
        Ok(User {
            id: ID::new(id),
            username: Username::new(username)?,
            email: Email::new(email)?,
            password: Password::new(password)?,
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
