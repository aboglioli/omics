mod preferences;
mod repository;
pub use preferences::*;
pub use repository::*;

use common::error::Error;
use common::model::{AggregateRoot, Events, StringId};
use common::result::Result;
use shared::event::ReaderEvent;

use crate::domain::collection::Collection;
use crate::domain::interaction::{
    CollectionFavorite, PublicationFavorite, ReaderCollectionId, ReaderPublicationId,
};
use crate::domain::publication::Publication;

pub type ReaderId = StringId;

#[derive(Debug, Clone)]
pub struct Reader {
    base: AggregateRoot<ReaderId>,
    events: Events<ReaderEvent>,
    subscribed: bool,
    preferences: Preferences,
}

impl Reader {
    pub fn new(id: ReaderId) -> Result<Self> {
        Ok(Reader {
            base: AggregateRoot::new(id),
            events: Events::new(),
            subscribed: false,
            preferences: Preferences::default(),
        })
    }

    pub fn build(base: AggregateRoot<ReaderId>, subscribed: bool) -> Self {
        Reader {
            base,
            events: Events::new(),
            subscribed,
            preferences: Preferences::default(),
        }
    }

    pub fn base(&self) -> &AggregateRoot<ReaderId> {
        &self.base
    }

    pub fn events(&self) -> &Events<ReaderEvent> {
        &self.events
    }

    pub fn is_subscribed(&self) -> bool {
        self.subscribed
    }

    pub fn preferences(&self) -> &Preferences {
        &self.preferences
    }

    pub fn preferences_mut(&mut self) -> &mut Preferences {
        // TODO: improve this
        self.base.update();
        &mut self.preferences
    }

    pub fn subscribe(&mut self) -> Result<()> {
        self.subscribed = true;
        self.base.update();
        Ok(())
    }

    pub fn unsubscribe(&mut self) -> Result<()> {
        self.subscribed = false;
        self.base.update();
        Ok(())
    }

    pub fn set_preferences(&mut self, preferences: Preferences) -> Result<()> {
        self.preferences = preferences;
        self.base.update();
        Ok(())
    }

    pub fn add_publication_to_favorites(
        &mut self,
        publication: &Publication,
    ) -> Result<PublicationFavorite> {
        if !publication.is_published() {
            return Err(Error::new("publication", "not_published"));
        }

        let favorite = PublicationFavorite::new(ReaderPublicationId::new(
            self.base().id().clone(),
            publication.base().id().clone(),
        )?)?;

        self.events
            .record_event(ReaderEvent::PublicationAddedToFavorites {
                reader_id: self.base().id().to_string(),
                publication_id: publication.base().id().to_string(),
            });

        Ok(favorite)
    }

    pub fn remove_publication_from_favorites(&mut self, publication: &Publication) -> Result<()> {
        self.events
            .record_event(ReaderEvent::PublicationRemovedFromFavorites {
                reader_id: self.base().id().to_string(),
                publication_id: publication.base().id().to_string(),
            });

        Ok(())
    }

    pub fn add_collection_to_favorites(
        &mut self,
        collection: &Collection,
    ) -> Result<CollectionFavorite> {
        let favorite = CollectionFavorite::new(ReaderCollectionId::new(
            self.base().id().clone(),
            collection.base().id().clone(),
        )?)?;

        self.events
            .record_event(ReaderEvent::CollectionAddedToFavorites {
                reader_id: self.base().id().to_string(),
                collection_id: collection.base().id().to_string(),
            });

        Ok(favorite)
    }

    pub fn remove_collection_from_favorites(&mut self, collection: &Collection) -> Result<()> {
        self.events
            .record_event(ReaderEvent::CollectionRemovedFromFavorites {
                reader_id: self.base().id().to_string(),
                collection_id: collection.base().id().to_string(),
            });

        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        self.base.delete();
        Ok(())
    }
}
