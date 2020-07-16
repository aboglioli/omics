use crate::common::error::Error;
use crate::common::model::Entity;
use crate::identity::domain::token::{Data, Token, TokenEncoder, TokenRepository, TokenService};
use crate::identity::domain::user::{PasswordHasher, User, UserRepository};

pub struct AuthenticationService<'a, UR, PH, TE, TR> {
    user_repository: &'a UR,
    password_hasher: &'a PH,
    token_service: &'a TokenService<TE, TR>,
}

impl<'a, UR, PH, TE, TR> AuthenticationService<'a, UR, PH, TE, TR>
where
    UR: UserRepository,
    PH: PasswordHasher,
    TE: TokenEncoder,
    TR: TokenRepository,
{
    pub fn new<'b>(
        user_repository: &'b UR,
        password_hasher: &'b PH,
        token_service: &'b TokenService<TE, TR>,
    ) -> AuthenticationService<'b, UR, PH, TE, TR> {
        AuthenticationService {
            user_repository,
            password_hasher,
            token_service,
        }
    }

    pub fn authenticate(
        &self,
        username_or_email: &str,
        password: &str,
    ) -> Result<(User, Token), Error> {
        let user = self
            .user_repository
            .find_by_username_or_email(username_or_email)?;
        if self
            .password_hasher
            .compare(user.password().value(), password)
        {
            let mut data = Data::new();
            data.add("user_id", &user.id().value());
            let token = self.token_service.create(data)?;

            return Ok((user, token));
        }
        Err(Error::application().set_code("invalid_credentials").clone())
    }
}
