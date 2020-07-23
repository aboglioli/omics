use crate::domain::role::RoleID;
use crate::domain::user::{Email, Identity, Password, Person, Provider, Username};
use common::error::Error;
use common::model::{Entity, ID};

// User
pub type UserID = String;

#[derive(Debug, Clone)]
pub struct User {
    id: ID<UserID>,
    identity: Identity,
    person: Option<Person>,
    role_id: RoleID,
    validated: bool,
}

impl User {
    pub fn new(id: UserID, identity: Identity, role_id: RoleID) -> Result<User, Error> {
        Ok(User {
            id: ID::new(id),
            identity,
            person: None,
            role_id,
            validated: false,
        })
    }

    pub fn identity(&self) -> &Identity {
        &self.identity
    }

    pub fn person(&self) -> Option<&Person> {
        self.person.as_ref()
    }

    pub fn role_id(&self) -> &RoleID {
        &self.role_id
    }

    pub fn is_validated(&self) -> bool {
        self.validated
    }

    pub fn is_active(&self) -> bool {
        self.id.deleted_at().is_none() && self.validated
    }

    pub fn set_password(&mut self, password: Password) -> Result<(), Error> {
        self.identity.set_password(password)?;
        Ok(())
    }

    pub fn set_person(&mut self, person: Person) -> Result<(), Error> {
        self.person = Some(person);
        Ok(())
    }

    pub fn set_role(&mut self, role_id: RoleID) {
        self.role_id = role_id
    }

    pub fn validate(&mut self) {
        self.validated = true;
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
        let user = User::new(
            UserID::from("user123"),
            Identity::new(
                Provider::new("local")?,
                Username::new("user1")?,
                Email::new("email@user.com")?,
                Some(Password::new(&format!("{:X>50}", "2"))?),
            )?,
            &role,
        )?;
        assert_eq!(user.identity().username().value(), "user1");
        assert_eq!(user.identity().email().value(), "email@user.com");

        Ok(())
    }
}
