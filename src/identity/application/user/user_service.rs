use std::rc::Rc;

use crate::common::error::Error;
use crate::common::event::EventPublisher;
use crate::common::model::Entity;
use crate::identity::application::user::{
    ChangePasswordCommand, LoginCommand, RegisterCommand, UpdateCommand,
};
use crate::identity::domain::role::RoleRepository;
use crate::identity::domain::token::{Token, TokenEncoder, TokenRepository};
use crate::identity::domain::user::{
    AuthenticationService, AuthorizationService, PasswordHasher, User, UserID, UserRegistered,
    UserRepository, UserUpdated,
};

pub struct UserService<
    TUserRepository,
    TEventPublisher,
    TPasswordHasher,
    TTokenEncoder,
    TTokenRepository,
    TRoleRepository,
> {
    user_repository: Rc<TUserRepository>,
    event_publisher: Rc<TEventPublisher>,
    authentication_service: Rc<
        AuthenticationService<TUserRepository, TPasswordHasher, TTokenEncoder, TTokenRepository>,
    >,
    authorization_service: Rc<AuthorizationService<TUserRepository, TPasswordHasher>>,
    role_repository: Rc<TRoleRepository>,
}

impl<
        TUserRepository,
        TEventPublisher,
        TPasswordHasher,
        TTokenEncoder,
        TTokenRepository,
        TRoleRepository,
    >
    UserService<
        TUserRepository,
        TEventPublisher,
        TPasswordHasher,
        TTokenEncoder,
        TTokenRepository,
        TRoleRepository,
    >
where
    TUserRepository: UserRepository,
    TEventPublisher: EventPublisher,
    TPasswordHasher: PasswordHasher,
    TTokenEncoder: TokenEncoder,
    TTokenRepository: TokenRepository,
    TRoleRepository: RoleRepository,
{
    pub fn new(
        user_repository: Rc<TUserRepository>,
        event_publisher: Rc<TEventPublisher>,
        authentication_service: Rc<
            AuthenticationService<
                TUserRepository,
                TPasswordHasher,
                TTokenEncoder,
                TTokenRepository,
            >,
        >,
        authorization_service: Rc<AuthorizationService<TUserRepository, TPasswordHasher>>,
        role_repository: Rc<TRoleRepository>,
    ) -> UserService<
        TUserRepository,
        TEventPublisher,
        TPasswordHasher,
        TTokenEncoder,
        TTokenRepository,
        TRoleRepository,
    > {
        UserService {
            user_repository,
            event_publisher,
            authentication_service,
            authorization_service,
            role_repository,
        }
    }

    pub fn get_by_id(&self, user_id: UserID) -> Result<User, Error> {
        let user = self.user_repository.find_by_id(user_id)?;
        Ok(user)
    }

    pub fn register(&self, cmd: RegisterCommand) -> Result<(), Error> {
        cmd.validate()?;

        self.authorization_service
            .available(&cmd.username, &cmd.email)?;
        let hashed_password = self
            .authorization_service
            .generate_password(&cmd.password)?;

        let mut user = User::new(
            self.user_repository.next_id()?,
            &cmd.username,
            &cmd.email,
            &hashed_password,
            &self.role_repository.get_by_code("user".to_owned())?,
        )?;

        self.user_repository.save(&mut user)?;

        let event = UserRegistered::new(
            user.id().value(),
            user.username().value(),
            user.email().value(),
        );
        self.event_publisher.publish("user.registered", event)?;

        Ok(())
    }

    pub fn login(&self, cmd: LoginCommand) -> Result<(User, Token), Error> {
        self.authentication_service
            .authenticate(&cmd.username_or_email, &cmd.password)
    }

    pub fn update(&self, user_id: UserID, cmd: UpdateCommand) -> Result<(), Error> {
        cmd.validate()?;

        let mut user = self.user_repository.find_by_id(user_id)?;

        user.change_name(&cmd.name, &cmd.lastname)?;

        self.user_repository.save(&mut user)?;

        if let Some(person) = user.person() {
            let event = UserUpdated::new(user.id().value(), person.name(), person.lastname());
            self.event_publisher.publish("user.updated", event)?;
        }

        Ok(())
    }

    pub fn change_password(
        &self,
        user_id: UserID,
        cmd: ChangePasswordCommand,
    ) -> Result<(), Error> {
        cmd.validate()?;
        self.authorization_service
            .change_password(user_id, &cmd.old_password, &cmd.new_password)
    }
}
