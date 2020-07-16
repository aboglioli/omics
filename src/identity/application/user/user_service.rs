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

pub struct UserService<'a, UR, EP, PH, TE, TR, RR> {
    user_repository: &'a UR,
    event_publisher: &'a EP,
    authentication_service: &'a AuthenticationService<'a, UR, PH, TE, TR>,
    authorization_service: &'a AuthorizationService<'a, UR, PH>,
    role_repository: &'a RR,
}

impl<'a, UR, EP, PH, TE, TR, RR> UserService<'_, UR, EP, PH, TE, TR, RR>
where
    UR: UserRepository,
    EP: EventPublisher,
    PH: PasswordHasher,
    TE: TokenEncoder,
    TR: TokenRepository,
    RR: RoleRepository,
{
    pub fn new<'b>(
        user_repository: &'b UR,
        event_publisher: &'b EP,
        authentication_service: &'b AuthenticationService<'_, UR, PH, TE, TR>,
        authorization_service: &'b AuthorizationService<'_, UR, PH>,
        role_repository: &'b RR,
    ) -> UserService<'b, UR, EP, PH, TE, TR, RR> {
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
