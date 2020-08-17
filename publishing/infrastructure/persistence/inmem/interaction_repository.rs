use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio::sync::Mutex;

use common::result::Result;

use crate::domain::interaction::{InteractionRepository, Like, Reading, Review, View};
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

pub struct InMemInteractionRepository {
    views: Mutex<Vec<View>>,
    readings: Mutex<Vec<Reading>>,
    likes: Mutex<Vec<Like>>,
    reviews: Mutex<Vec<Review>>,
}

impl InMemInteractionRepository {
    pub fn new() -> Self {
        InMemInteractionRepository {
            views: Mutex::new(Vec::new()),
            readings: Mutex::new(Vec::new()),
            likes: Mutex::new(Vec::new()),
            reviews: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl InteractionRepository for InMemInteractionRepository {
    async fn find_views(
        &self,
        reader_id: Option<&ReaderId>,
        publication_id: Option<&PublicationId>,
        _from: Option<&DateTime<Utc>>,
        _to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<View>> {
        Ok(self
            .views
            .lock()
            .await
            .iter()
            .filter(|view| {
                if let Some(reader_id) = reader_id {
                    if view.base().reader_id() != reader_id {
                        return false;
                    }
                }

                if let Some(publication_id) = publication_id {
                    if view.base().publication_id() != publication_id {
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
        _from: Option<&DateTime<Utc>>,
        _to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Reading>> {
        Ok(self
            .readings
            .lock()
            .await
            .iter()
            .filter(|reading| {
                if let Some(reader_id) = reader_id {
                    if reading.reader_id() != reader_id {
                        return false;
                    }
                }

                if let Some(publication_id) = publication_id {
                    if reading.publication_id() != publication_id {
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
        _from: Option<&DateTime<Utc>>,
        _to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Like>> {
        Ok(self
            .likes
            .lock()
            .await
            .iter()
            .filter(|like| {
                if let Some(reader_id) = reader_id {
                    if like.reader_id() != reader_id {
                        return false;
                    }
                }

                if let Some(publication_id) = publication_id {
                    if like.publication_id() != publication_id {
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
        _from: Option<&DateTime<Utc>>,
        _to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Review>> {
        Ok(self
            .reviews
            .lock()
            .await
            .iter()
            .filter(|review| {
                if let Some(reader_id) = reader_id {
                    if review.base().reader_id() != reader_id {
                        return false;
                    }
                }

                if let Some(publication_id) = publication_id {
                    if review.base().publication_id() != publication_id {
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

    async fn delete_like(
        &self,
        reader_id: &ReaderId,
        publication_id: &PublicationId,
    ) -> Result<()> {
        self.likes.lock().await.retain(|like| {
            like.reader_id() != reader_id && like.publication_id() != publication_id
        });
        Ok(())
    }

    async fn delete_review(
        &self,
        reader_id: &ReaderId,
        publication_id: &PublicationId,
    ) -> Result<()> {
        self.reviews.lock().await.retain(|review| {
            review.base().reader_id() != reader_id
                && review.base().publication_id() != publication_id
        });
        Ok(())
    }
}
