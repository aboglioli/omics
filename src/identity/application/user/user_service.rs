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

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use crate::common::error::Error;
    use crate::common::model::Entity;
    use crate::identity::domain::{
        role::Role,
        token::TokenService,
        user::{AuthenticationService, AuthorizationService, User, UserRepository},
    };
    use crate::identity::infrastructure::{mocks::*, persistence::inmem::*};

    struct Container {
        user_repo: Rc<InMemUserRepository>,
        event_pub: Rc<InMemEventPublisher>,
        password_hasher: Rc<FakePasswordHasher>,
        token_serv: Rc<TokenService<FakeTokenEncoder, InMemTokenRepository>>,
        authentication_serv: Rc<
            AuthenticationService<
                InMemUserRepository,
                FakePasswordHasher,
                FakeTokenEncoder,
                InMemTokenRepository,
            >,
        >,
        authorization_serv: Rc<AuthorizationService<InMemUserRepository, FakePasswordHasher>>,
        role_repo: Rc<InMemRoleRepository>,
        user_serv: UserService<
            InMemUserRepository,
            InMemEventPublisher,
            FakePasswordHasher,
            FakeTokenEncoder,
            InMemTokenRepository,
            InMemRoleRepository,
        >,
    }

    impl Container {
        fn new() -> Container {
            let user_repo = Rc::new(InMemUserRepository::new());
            let event_pub = Rc::new(InMemEventPublisher::new());
            let password_hasher = Rc::new(FakePasswordHasher::new());
            let token_serv = Rc::new(TokenService::new(
                FakeTokenEncoder::new(),
                InMemTokenRepository::new(),
            ));
            let authentication_serv = Rc::new(AuthenticationService::new(
                Rc::clone(&user_repo),
                Rc::clone(&password_hasher),
                Rc::clone(&token_serv),
            ));
            let authorization_serv = Rc::new(AuthorizationService::new(
                Rc::clone(&user_repo),
                Rc::clone(&password_hasher),
            ));
            let role_repo = Rc::new(InMemRoleRepository::new());

            let user_serv = UserService::new(
                Rc::clone(&user_repo),
                Rc::clone(&event_pub),
                Rc::clone(&authentication_serv),
                Rc::clone(&authorization_serv),
                Rc::clone(&role_repo),
            );

            Container {
                user_repo,
                event_pub,
                password_hasher,
                token_serv,
                authentication_serv,
                authorization_serv,
                role_repo,
                user_serv,
            }
        }
    }

    #[test]
    fn get_by_id() -> Result<(), Error> {
        let c = Container::new();

        let user_id = c.user_repo.next_id()?;
        let mut user = User::new(
            user_id.clone(),
            "user12",
            "user@email.com",
            &c.authorization_serv.generate_password("user123")?,
            &Role::new("user".to_owned(), "User")?,
        )?;
        c.user_repo.save(&mut user)?;

        let found_user = c.user_serv.get_by_id(user_id)?;
        assert_eq!(found_user.id(), user.id());
        assert_eq!(found_user.username().value(), "user12");
        assert_eq!(found_user.email().value(), "user@email.com");

        Ok(())
    }
}
