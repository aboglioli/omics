use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::collection::CollectionId;
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

#[derive(Debug, Clone)]
pub struct ReaderPublicationId {
    reader_id: ReaderId,
    publication_id: PublicationId,
}

impl ReaderPublicationId {
    pub fn new(reader_id: ReaderId, publication_id: PublicationId) -> Result<Self> {
        Ok(ReaderPublicationId {
            reader_id,
            publication_id,
        })
    }

    pub fn reader_id(&self) -> &ReaderId {
        &self.reader_id
    }

    pub fn publication_id(&self) -> &PublicationId {
        &self.publication_id
    }
}

#[derive(Debug, Clone)]
pub struct ReaderCollectionId {
    reader_id: ReaderId,
    collection_id: CollectionId,
}

impl ReaderCollectionId {
    pub fn new(reader_id: ReaderId, collection_id: CollectionId) -> Result<Self> {
        Ok(ReaderCollectionId {
            reader_id,
            collection_id,
        })
    }

    pub fn reader_id(&self) -> &ReaderId {
        &self.reader_id
    }

    pub fn collection_id(&self) -> &CollectionId {
        &self.collection_id
    }
}

#[derive(Debug, Clone)]
pub struct ReaderAuthorId {
    reader_id: ReaderId,
    author_id: AuthorId,
}

impl ReaderAuthorId {
    pub fn new(reader_id: ReaderId, author_id: AuthorId) -> Result<Self> {
        Ok(ReaderAuthorId {
            reader_id,
            author_id,
        })
    }

    pub fn reader_id(&self) -> &ReaderId {
        &self.reader_id
    }

    pub fn author_id(&self) -> &AuthorId {
        &self.author_id
    }
}
