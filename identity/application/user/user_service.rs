use std::rc::Rc;

use common::error::Error;
use common::event::EventPublisher;

use crate::application::user::{
    ChangePasswordCommand, LoginCommand, RegisterCommand, UpdateCommand,
};
use crate::domain::role::{RoleId, RoleRepository};
use crate::domain::token::Token;
use crate::domain::user::{
    AuthService, Email, Fullname, Identity, Password, Person, Provider, User, UserId,
    UserRegistered, UserRepository, UserUpdated, Username,
};
use crate::domain::validation::{Validation, ValidationCode, ValidationRepository};

pub struct UserService {
    user_repository: Rc<dyn UserRepository>,
    event_publisher: Rc<dyn EventPublisher>,
    auth_serv: Rc<AuthService>,
    role_repository: Rc<dyn RoleRepository>,
    validation_repository: Rc<dyn ValidationRepository>,
}

impl UserService {
    pub fn new(
        user_repository: Rc<dyn UserRepository>,
        event_publisher: Rc<dyn EventPublisher>,
        auth_serv: Rc<AuthService>,
        role_repository: Rc<dyn RoleRepository>,
        validation_repository: Rc<dyn ValidationRepository>,
    ) -> Self {
        UserService {
            user_repository,
            event_publisher,
            auth_serv,
            role_repository,
            validation_repository,
        }
    }

    pub fn get_by_id(&self, user_id: &UserId) -> Result<User, Error> {
        let user = self.user_repository.find_by_id(user_id)?;
        Ok(user)
    }

    pub fn register(&self, cmd: RegisterCommand) -> Result<(), Error> {
        cmd.validate()?;

        self.auth_serv.available(&cmd.username, &cmd.email)?;
        let hashed_password = self.auth_serv.generate_password(&cmd.password)?;

        let mut user = User::new(
            self.user_repository.next_id()?,
            Identity::new(
                Provider::Local,
                Username::new(&cmd.username)?,
                Email::new(&cmd.email)?,
                Some(Password::new(&hashed_password)?),
            )?,
            RoleId::from("user"),
        )?;

        self.user_repository.save(&mut user)?;

        let event = UserRegistered::new(
            user.base().id(),
            user.identity().username().value(),
            user.identity().email().value(),
        );
        self.event_publisher
            .publish("user.registered", Box::new(event))?;

        Ok(())
    }

    pub fn login(&self, cmd: LoginCommand) -> Result<Token, Error> {
        self.auth_serv
            .authenticate(&cmd.username_or_email, &cmd.password)
    }

    pub fn update(&self, user_id: &UserId, cmd: UpdateCommand) -> Result<(), Error> {
        cmd.validate()?;

        let mut user = self.user_repository.find_by_id(&user_id)?;

        let person = Person::new(Fullname::new(&cmd.name, &cmd.lastname)?)?;
        user.set_person(person);
        self.user_repository.save(&mut user)?;

        if let Some(person) = user.person() {
            let event = UserUpdated::new(
                user.base().id(),
                person.fullname().name(),
                person.fullname().lastname(),
            );
            self.event_publisher
                .publish("user.updated", Box::new(event))?;
        }

        Ok(())
    }

    pub fn change_password(
        &self,
        user_id: &UserId,
        cmd: ChangePasswordCommand,
    ) -> Result<(), Error> {
        cmd.validate()?;
        self.auth_serv
            .change_password(user_id, &cmd.old_password, &cmd.new_password)
    }

    pub fn validate_user(
        &self,
        user_id: &UserId,
        validation_code: &ValidationCode,
    ) -> Result<(), Error> {
        let mut user = self.user_repository.find_by_id(user_id)?;
        let mut validation = self.validation_repository.find_by_code(validation_code)?;
        validation.validate_user(&mut user, validation_code)?;

        self.user_repository.save(&mut user)?;
        self.validation_repository.save(&mut validation)?;

        Ok(())
    }
}
