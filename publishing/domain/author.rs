mod repository;
pub use repository::*;

use common::error::Error;
use common::model::{AggregateRoot, Events, StringId};
use common::result::Result;
use shared::event::AuthorEvent;

use crate::domain::interaction::{Follow, ReaderAuthorId};
use crate::domain::reader::Reader;

pub type AuthorId = StringId;

#[derive(Debug, Clone)]
pub struct Author {
    base: AggregateRoot<AuthorId>,
    events: Events<AuthorEvent>,
    username: String,
    name: Option<String>,
    lastname: Option<String>,
    biography: Option<String>,
    profile_image: Option<String>,
    followers: u32,
    publications: u32,
}

impl Author {
    pub fn new<S: Into<String>>(id: AuthorId, username: S) -> Result<Self> {
        Ok(Author {
            base: AggregateRoot::new(id),
            events: Events::new(),
            username: username.into(),
            name: None,
            lastname: None,
            biography: None,
            profile_image: None,
            followers: 0,
            publications: 0,
        })
    }

    pub fn build(
        base: AggregateRoot<AuthorId>,
        username: String,
        name: Option<String>,
        lastname: Option<String>,
        biography: Option<String>,
        profile_image: Option<String>,
        followers: u32,
        publications: u32,
    ) -> Self {
        Author {
            base,
            events: Events::new(),
            username,
            name,
            lastname,
            biography,
            profile_image,
            followers,
            publications,
        }
    }

    pub fn base(&self) -> &AggregateRoot<AuthorId> {
        &self.base
    }

    pub fn events(&self) -> &Events<AuthorEvent> {
        &self.events
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

    pub fn followers(&self) -> u32 {
        self.followers
    }

    pub fn publications(&self) -> u32 {
        self.publications
    }

    pub fn set_name<S: Into<String>>(&mut self, name: S, lastname: S) -> Result<()> {
        self.name = Some(name.into());
        self.lastname = Some(lastname.into());
        Ok(())
    }

    pub fn set_biography<S: Into<String>>(&mut self, biography: S) -> Result<()> {
        self.biography = Some(biography.into());
        Ok(())
    }

    pub fn set_profile_image<S: Into<String>>(&mut self, profile_image: S) -> Result<()> {
        self.profile_image = Some(profile_image.into());
        Ok(())
    }

    pub fn follow(&mut self, reader: &Reader) -> Result<Follow> {
        if self.base().id() == reader.base().id() {
            return Err(Error::new("author", "cannot_follow_itself"));
        }

        let follow = Follow::new(ReaderAuthorId::new(
            reader.base().id().clone(),
            self.base().id().clone(),
        )?)?;

        self.followers += 1;
        self.base.update();

        self.events.record_event(AuthorEvent::Followed {
            author_id: self.base().id().to_string(),
            reader_id: reader.base().id().to_string(),
        });

        Ok(follow)
    }

    pub fn unfollow(&mut self, reader: &Reader) -> Result<()> {
        if self.base().id() == reader.base().id() {
            return Err(Error::new("author", "cannot_unfollow_itself"));
        }

        if self.followers == 0 {
            return Err(Error::new("author", "does_not_have_followers"));
        }

        self.followers -= 1;
        self.base.update();

        self.events.record_event(AuthorEvent::Unfollowed {
            author_id: self.base().id().to_string(),
            reader_id: reader.base().id().to_string(),
        });

        Ok(())
    }

    pub fn set_publications(&mut self, publications: u32) -> Result<()> {
        self.publications = publications;
        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        self.base.delete();
        Ok(())
    }
}
