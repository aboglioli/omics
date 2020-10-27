use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::row::Row;
use tokio_postgres::Client;
use uuid::Uuid;

use common::error::Error;
use common::model::{AggregateRoot, Pagination};
use common::result::Result;
use common::sql::where_builder::WhereBuilder;

use crate::domain::role::RoleId;
use crate::domain::user::{
    Biography, Birthdate, Email, Fullname, Gender, Identity, Image, Password, Person, Provider,
    User, UserId, UserOrderBy, UserRepository, Username, Validation,
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

        let payment_email: Option<String> = row.get("payment_email");

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
            password.map(Password::new).transpose()?,
        )?;

        let person = if name.is_some() && lastname.is_some() {
            Some(Person::new(
                Fullname::new(name.unwrap(), lastname.unwrap())?,
                birthdate.map(Birthdate::new).transpose()?,
                gender.map(|g| Gender::from_str(&g)).transpose()?,
                biography.map(Biography::new).transpose()?,
                profile_image.map(Image::new).transpose()?,
            )?)
        } else {
            None
        };

        let role_id = RoleId::new(role_id)?;
        let validation = validation_code.map(Validation::build);

        Ok(User::build(
            agg_root,
            identity,
            person,
            role_id,
            validation,
            payment_email.map(Email::new).transpose()?,
        ))
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

    async fn search(
        &self,
        role_id: Option<&RoleId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
        offset: Option<usize>,
        limit: Option<usize>,
        order_by: Option<&UserOrderBy>,
    ) -> Result<Pagination<User>> {
        let role_id = role_id.map(|id| id.value());

        let (sql, params) = WhereBuilder::new()
            .add_param_opt("role_id $$", &role_id, role_id.is_some())
            .add_param_opt("created_at >= $$", &from, from.is_some())
            .add_param_opt("created_at <= $$", &to, to.is_some())
            .build();

        // Total
        let row = self
            .client
            .query_one("SELECT COUNT(*) FROM users", &[])
            .await
            .map_err(|err| Error::new("user", "total").wrap_raw(err))?;
        let total: i64 = row.get(0);

        // Matching criteria
        let row = self
            .client
            .query_one(
                &format!(
                    "SELECT COUNT(*) FROM users
                    {}",
                    sql,
                ) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::new("user", "matching_criteria").wrap_raw(err))?;
        let matching_criteria: i64 = row.get(0);

        // Query
        let offset = offset.unwrap_or_else(|| 0);
        let limit = limit.unwrap_or_else(|| total as usize);
        let order_by = match order_by {
            Some(UserOrderBy::Newest) => "created_at DESC",
            _ => "created_at ASC",
        };

        let rows = self
            .client
            .query(
                &format!(
                    "SELECT * FROM users
                    {}
                    ORDER BY {}
                    OFFSET {}
                    LIMIT {}",
                    sql, order_by, offset, limit,
                ) as &str,
                &params,
            )
            .await
            .map_err(|err| Error::not_found("user").wrap_raw(err))?;

        let mut users = Vec::new();
        for row in rows.into_iter() {
            users.push(User::from_row(row)?);
        }

        Ok(
            Pagination::new(offset, limit, total as usize, matching_criteria as usize)
                .add_items(users),
        )
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
                        payment_email = $11,
                        updated_at = $12,
                        deleted_at = $13
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
                        &user.payment_email().map(|e| e.to_string()),
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
