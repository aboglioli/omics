use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio::sync::Mutex;

use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::collection::CollectionId;
use crate::domain::interaction::{
    CollectionFavorite, Follow, InteractionRepository, Like, PublicationFavorite, Reading, Review,
    View,
};
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

pub struct InMemInteractionRepository {
    views: Mutex<Vec<View>>,
    readings: Mutex<Vec<Reading>>,
    likes: Mutex<Vec<Like>>,
    reviews: Mutex<Vec<Review>>,
    publication_favorites: Mutex<Vec<PublicationFavorite>>,
    collection_favorites: Mutex<Vec<CollectionFavorite>>,
    follows: Mutex<Vec<Follow>>,
}

impl InMemInteractionRepository {
    pub fn new() -> Self {
        InMemInteractionRepository {
            views: Mutex::new(Vec::new()),
            readings: Mutex::new(Vec::new()),
            likes: Mutex::new(Vec::new()),
            reviews: Mutex::new(Vec::new()),
            publication_favorites: Mutex::new(Vec::new()),
            collection_favorites: Mutex::new(Vec::new()),
            follows: Mutex::new(Vec::new()),
        }
    }
}

impl Default for InMemInteractionRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl InteractionRepository for InMemInteractionRepository {
    async fn find_views(
        &self,
        reader_id: Option<&ReaderId>,
        publication_id: Option<&PublicationId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<View>> {
        Ok(self
            .views
            .lock()
            .await
            .iter()
            .filter(|view| {
                if let Some(reader_id) = reader_id {
                    if view.base().id().reader_id() != reader_id {
                        return false;
                    }
                }

                if let Some(publication_id) = publication_id {
                    if view.base().id().publication_id() != publication_id {
                        return false;
                    }
                }

                if let Some(from) = from {
                    if view.base().created_at() < from {
                        return false;
                    }
                }

                if let Some(to) = to {
                    if view.base().created_at() > to {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect())
    }

    async fn find_readings(
        &self,
        reader_id: Option<&ReaderId>,
        publication_id: Option<&PublicationId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Reading>> {
        Ok(self
            .readings
            .lock()
            .await
            .iter()
            .filter(|reading| {
                if let Some(reader_id) = reader_id {
                    if reading.base().id().reader_id() != reader_id {
                        return false;
                    }
                }

                if let Some(publication_id) = publication_id {
                    if reading.base().id().publication_id() != publication_id {
                        return false;
                    }
                }

                if let Some(from) = from {
                    if reading.base().created_at() < from {
                        return false;
                    }
                }

                if let Some(to) = to {
                    if reading.base().created_at() > to {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect())
    }

    async fn find_likes(
        &self,
        reader_id: Option<&ReaderId>,
        publication_id: Option<&PublicationId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Like>> {
        Ok(self
            .likes
            .lock()
            .await
            .iter()
            .filter(|like| {
                if let Some(reader_id) = reader_id {
                    if like.base().id().reader_id() != reader_id {
                        return false;
                    }
                }

                if let Some(publication_id) = publication_id {
                    if like.base().id().publication_id() != publication_id {
                        return false;
                    }
                }

                if let Some(from) = from {
                    if like.base().created_at() < from {
                        return false;
                    }
                }

                if let Some(to) = to {
                    if like.base().created_at() > to {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect())
    }

    async fn find_reviews(
        &self,
        reader_id: Option<&ReaderId>,
        publication_id: Option<&PublicationId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Review>> {
        Ok(self
            .reviews
            .lock()
            .await
            .iter()
            .filter(|review| {
                if let Some(reader_id) = reader_id {
                    if review.base().id().reader_id() != reader_id {
                        return false;
                    }
                }

                if let Some(publication_id) = publication_id {
                    if review.base().id().publication_id() != publication_id {
                        return false;
                    }
                }

                if let Some(from) = from {
                    if review.base().created_at() < from {
                        return false;
                    }
                }

                if let Some(to) = to {
                    if review.base().created_at() > to {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect())
    }

    async fn find_publication_favorites(
        &self,
        reader_id: Option<&ReaderId>,
        publication_id: Option<&PublicationId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<PublicationFavorite>> {
        Ok(self
            .publication_favorites
            .lock()
            .await
            .iter()
            .filter(|favorite| {
                if let Some(reader_id) = reader_id {
                    if favorite.base().id().reader_id() != reader_id {
                        return false;
                    }
                }

                if let Some(publication_id) = publication_id {
                    if favorite.base().id().publication_id() != publication_id {
                        return false;
                    }
                }

                if let Some(from) = from {
                    if favorite.base().created_at() < from {
                        return false;
                    }
                }

                if let Some(to) = to {
                    if favorite.base().created_at() > to {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect())
    }

    async fn find_collection_favorites(
        &self,
        reader_id: Option<&ReaderId>,
        collection_id: Option<&CollectionId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<CollectionFavorite>> {
        Ok(self
            .collection_favorites
            .lock()
            .await
            .iter()
            .filter(|favorite| {
                if let Some(reader_id) = reader_id {
                    if favorite.base().id().reader_id() != reader_id {
                        return false;
                    }
                }

                if let Some(collection_id) = collection_id {
                    if favorite.base().id().collection_id() != collection_id {
                        return false;
                    }
                }

                if let Some(from) = from {
                    if favorite.base().created_at() < from {
                        return false;
                    }
                }

                if let Some(to) = to {
                    if favorite.base().created_at() > to {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect())
    }

    async fn find_follows(
        &self,
        reader_id: Option<&ReaderId>,
        author_id: Option<&AuthorId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Follow>> {
        Ok(self
            .follows
            .lock()
            .await
            .iter()
            .filter(|follow| {
                if let Some(reader_id) = reader_id {
                    if follow.base().id().reader_id() != reader_id {
                        return false;
                    }
                }

                if let Some(author_id) = author_id {
                    if follow.base().id().author_id() != author_id {
                        return false;
                    }
                }

                if let Some(from) = from {
                    if follow.base().created_at() < from {
                        return false;
                    }
                }

                if let Some(to) = to {
                    if follow.base().created_at() > to {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect())
    }

    async fn save_view(&self, view: &mut View) -> Result<()> {
        self.views.lock().await.push(view.clone());
        Ok(())
    }

    async fn save_reading(&self, reading: &mut Reading) -> Result<()> {
        self.readings.lock().await.push(reading.clone());
        Ok(())
    }

    async fn save_like(&self, like: &mut Like) -> Result<()> {
        self.likes.lock().await.push(like.clone());
        Ok(())
    }

    async fn save_review(&self, review: &mut Review) -> Result<()> {
        self.reviews.lock().await.push(review.clone());
        Ok(())
    }

    async fn save_publication_favorite(&self, favorite: &mut PublicationFavorite) -> Result<()> {
        self.publication_favorites
            .lock()
            .await
            .push(favorite.clone());
        Ok(())
    }

    async fn save_collection_favorite(&self, favorite: &mut CollectionFavorite) -> Result<()> {
        self.collection_favorites
            .lock()
            .await
            .push(favorite.clone());
        Ok(())
    }

    async fn save_follow(&self, follow: &mut Follow) -> Result<()> {
        self.follows.lock().await.push(follow.clone());
        Ok(())
    }

    async fn delete_like(
        &self,
        reader_id: &ReaderId,
        publication_id: &PublicationId,
    ) -> Result<()> {
        self.likes.lock().await.retain(|like| {
            like.base().id().reader_id() != reader_id
                && like.base().id().publication_id() != publication_id
        });
        Ok(())
    }

    async fn delete_review(
        &self,
        reader_id: &ReaderId,
        publication_id: &PublicationId,
    ) -> Result<()> {
        self.reviews.lock().await.retain(|review| {
            review.base().id().reader_id() != reader_id
                && review.base().id().publication_id() != publication_id
        });
        Ok(())
    }

    async fn delete_publication_favorite(
        &self,
        reader_id: &ReaderId,
        publication_id: &PublicationId,
    ) -> Result<()> {
        self.publication_favorites.lock().await.retain(|favorite| {
            favorite.base().id().reader_id() != reader_id
                && favorite.base().id().publication_id() != publication_id
        });
        Ok(())
    }

    async fn delete_collection_favorite(
        &self,
        reader_id: &ReaderId,
        collection_id: &CollectionId,
    ) -> Result<()> {
        self.collection_favorites.lock().await.retain(|favorite| {
            favorite.base().id().reader_id() != reader_id
                && favorite.base().id().collection_id() != collection_id
        });
        Ok(())
    }

    async fn delete_follow(&self, reader_id: &ReaderId, author_id: &AuthorId) -> Result<()> {
        self.follows.lock().await.retain(|follow| {
            follow.base().id().reader_id() != reader_id
                && follow.base().id().author_id() != author_id
        });
        Ok(())
    }
}
