mod repository;
mod service;
pub use repository::*;
pub use service::*;

use common::event::Event;
use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type UserId = StringId;

#[derive(Debug, Clone)]
pub struct User {
    base: AggregateRoot<UserId, Event>,
    username: String,
    name: Option<String>,
    lastname: Option<String>,
    biography: Option<String>,
    profile_image: Option<String>,
    role_id: String,
}

impl User {
    pub fn new<S: Into<String>>(id: UserId, username: S, role_id: S) -> Result<Self> {
        Ok(User {
            base: AggregateRoot::new(id),
            username: username.into(),
            name: None,
            lastname: None,
            biography: None,
            profile_image: None,
            role_id: role_id.into(),
        })
    }

    pub fn build(
        base: AggregateRoot<UserId, Event>,
        username: String,
        name: Option<String>,
        lastname: Option<String>,
        biography: Option<String>,
        profile_image: Option<String>,
        role_id: String,
    ) -> Self {
        User {
            base,
            username,
            name,
            lastname,
            biography,
            profile_image,
            role_id,
        }
    }

    pub fn base(&self) -> &AggregateRoot<UserId, Event> {
        &self.base
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn lastname(&self) -> Option<&String> {
        self.lastname.as_ref()
    }

    pub fn biography(&self) -> Option<&String> {
        self.biography.as_ref()
    }

    pub fn profile_image(&self) -> Option<&String> {
        self.profile_image.as_ref()
    }

    pub fn role_id(&self) -> &str {
        &self.role_id
    }

    pub fn is_admin(&self) -> bool {
        self.role_id == "admin"
    }

    pub fn is_content_manager(&self) -> bool {
        self.role_id == "admin" || self.role_id == "content-manager"
    }

    pub fn set_name<S: Into<String>>(&mut self, name: S, lastname: S) -> Result<()> {
        self.name = Some(name.into());
        self.lastname = Some(lastname.into());
        self.base.update();
        Ok(())
    }

    pub fn set_biography<S: Into<String>>(&mut self, biography: S) -> Result<()> {
        self.biography = Some(biography.into());
        self.base.update();
        Ok(())
    }

    pub fn set_profile_image<S: Into<String>>(&mut self, profile_image: S) -> Result<()> {
        self.profile_image = Some(profile_image.into());
        self.base.update();
        Ok(())
    }

    pub fn change_role(&mut self, role_id: String) -> Result<()> {
        self.role_id = role_id;
        self.base.update();
        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        self.base.delete();
        Ok(())
    }
}
