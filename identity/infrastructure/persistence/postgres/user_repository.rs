use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::row::Row;
use tokio_postgres::Client;
use uuid::Uuid;

use common::error::Error;
use common::model::AggregateRoot;
use common::result::Result;

use crate::domain::role::RoleId;
use crate::domain::user::{
    Biography, Birthdate, Email, Fullname, Gender, Identity, Image, Password, Person, Provider,
    User, UserId, UserRepository, Username, Validation,
};

impl User {
    fn from_row(row: Row) -> Result<Self> {
        let id: Uuid = row.get("id");

        let provider: String = row.get("provider");
        let username: String = row.get("username");
        let email: String = row.get("email");
        let password: Option<String> = row.get("password");

        let name: Option<String> = row.get("name");
        let lastname: Option<String> = row.get("lastname");
        let birthdate: Option<DateTime<Utc>> = row.get("birthdate");
        let gender: Option<String> = row.get("gender");
        let biography: Option<String> = row.get("biography");
        let profile_image: Option<String> = row.get("profile_image");

        let role_id: String = row.get("role_id");

        let validation_code: Option<String> = row.get("validation_code");

        let created_at: DateTime<Utc> = row.get("created_at");
        let updated_at: Option<DateTime<Utc>> = row.get("updated_at");
        let deleted_at: Option<DateTime<Utc>> = row.get("deleted_at");

        let agg_root = AggregateRoot::build(
            UserId::new(id.to_string())?,
            created_at,
            updated_at,
            deleted_at,
        );

        let identity = Identity::new(
            Provider::from_str(&provider)?,
            Username::new(username)?,
            Email::new(email)?,
            password.map(|p| Password::new(p)).transpose()?,
        )?;

        let person = if name.is_some() && lastname.is_some() {
            Some(Person::new(
                Fullname::new(name.unwrap(), lastname.unwrap())?,
                birthdate.map(|b| Birthdate::new(b)).transpose()?,
                gender.map(|g| Gender::from_str(&g)).transpose()?,
                biography.map(|b| Biography::new(b)).transpose()?,
                profile_image.map(|pi| Image::new(pi)).transpose()?,
            )?)
        } else {
            None
        };

        let role_id = RoleId::new(role_id)?;
        let validation = validation_code.map(|c| Validation::build(c));

        Ok(User::build(agg_root, identity, person, role_id, validation))
    }
}

pub struct PostgresUserRepository {
    client: Arc<Client>,
}

impl PostgresUserRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgresUserRepository { client }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_all(&self) -> Result<Vec<User>> {
        let rows = self
            .client
            .query("SELECT * FROM users", &[])
            .await
            .map_err(|err| Error::not_found("user").wrap_raw(err))?;

        let mut users = Vec::new();

        for row in rows.into_iter() {
            users.push(User::from_row(row)?);
        }

        Ok(users)
    }

    async fn find_by_id(&self, id: &UserId) -> Result<User> {
        let row = self
            .client
            .query_one(
                "SELECT * FROM users
                WHERE id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::not_found("user").wrap_raw(err))?;

        User::from_row(row)
    }

    async fn find_by_username(&self, username: &Username) -> Result<User> {
        let row = self
            .client
            .query_one(
                "SELECT * FROM users
                WHERE username = $1",
                &[&username.value()],
            )
            .await
            .map_err(|err| Error::not_found("user").wrap_raw(err))?;

        User::from_row(row)
    }

    async fn find_by_email(&self, email: &Email) -> Result<User> {
        let row = self
            .client
            .query_one(
                "SELECT * FROM users
                WHERE email = $1",
                &[&email.value()],
            )
            .await
            .map_err(|err| Error::not_found("user").wrap_raw(err))?;

        User::from_row(row)
    }

    async fn find_by_role_id(&self, role_id: &RoleId) -> Result<Vec<User>> {
        let rows = self
            .client
            .query(
                "SELECT * FROM users
                WHERE role_id = $1",
                &[&role_id.value()],
            )
            .await
            .map_err(|err| Error::not_found("user").wrap_raw(err))?;

        let mut users = Vec::new();

        for row in rows.into_iter() {
            users.push(User::from_row(row)?);
        }

        Ok(users)
    }

    async fn save(&self, user: &mut User) -> Result<()> {
        let create = self
            .client
            .query_one(
                "SELECT * FROM users WHERE id = $1",
                &[&user.base().id().to_uuid()?],
            )
            .await
            .is_err();

        if create {
            self.client
                .execute(
                    "INSERT INTO users(
                        id,
                        provider,
                        username,
                        email,
                        password,
                        name,
                        lastname,
                        birthdate,
                        gender,
                        biography,
                        profile_image,
                        role_id,
                        validation_code,
                        created_at,
                        updated_at,
                        deleted_at
                    ) VALUES (
                        $1,
                        $2,
                        $3,
                        $4,
                        $5,
                        $6,
                        $7,
                        $8,
                        $9,
                        $10,
                        $11,
                        $12,
                        $13,
                        $14,
                        $15,
                        $16
                    )",
                    &[
                        &user.base().id().to_uuid()?,
                        &user.identity().provider().to_string(),
                        &user.identity().username().value(),
                        &user.identity().email().value(),
                        &user.identity().password().map(|p| p.value()),
                        &user.person().map(|p| p.fullname().name()),
                        &user.person().map(|p| p.fullname().lastname()),
                        &user
                            .person()
                            .map(|p| p.birthdate().map(|b| b.date()))
                            .flatten(),
                        &user
                            .person()
                            .map(|p| p.gender().map(|g| g.to_string()))
                            .flatten(),
                        &user
                            .person()
                            .map(|p| p.biography().map(|b| b.value()))
                            .flatten(),
                        &user
                            .person()
                            .map(|p| p.profile_image().map(|pi| pi.url()))
                            .flatten(),
                        &user.role_id().value(),
                        &user.validation().map(|v| v.code()),
                        &user.base().created_at(),
                        &user.base().updated_at(),
                        &user.base().deleted_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("user", "create").wrap_raw(err))?;
        } else {
            self.client
                .execute(
                    "UPDATE users
                    SET
                        password = $2,
                        name = $3,
                        lastname = $4,
                        birthdate = $5,
                        gender = $6,
                        biography = $7,
                        profile_image = $8,
                        role_id = $9,
                        validation_code = $10,
                        updated_at = $11,
                        deleted_at = $12
                    WHERE
                        id = $1",
                    &[
                        &user.base().id().to_uuid()?,
                        &user.identity().password().map(|p| p.value()),
                        &user.person().map(|p| p.fullname().name()),
                        &user.person().map(|p| p.fullname().lastname()),
                        &user
                            .person()
                            .map(|p| p.birthdate().map(|b| b.date()))
                            .flatten(),
                        &user
                            .person()
                            .map(|p| p.gender().map(|g| g.to_string()))
                            .flatten(),
                        &user
                            .person()
                            .map(|p| p.biography().map(|b| b.value()))
                            .flatten(),
                        &user
                            .person()
                            .map(|p| p.profile_image().map(|pi| pi.url()))
                            .flatten(),
                        &user.role_id().value(),
                        &user.validation().map(|v| v.code()),
                        &user.base().updated_at(),
                        &user.base().deleted_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("user", "update").wrap_raw(err))?;
        }

        Ok(())
    }

    async fn delete(&self, id: &UserId) -> Result<()> {
        self.client
            .execute(
                "DELETE FROM users
                WHERE id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|err| Error::new("user", "delete").wrap_raw(err))?;

        Ok(())
    }
}
