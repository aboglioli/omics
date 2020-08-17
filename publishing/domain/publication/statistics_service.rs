use std::sync::Arc;

use chrono::{DateTime, Utc};

use common::result::Result;

use crate::domain::interaction::{InteractionRepository, Like, Reading, Review, View};
use crate::domain::publication::{PublicationId, Statistics};
use crate::domain::reader::ReaderId;

pub struct StatisticsService<IRepo> {
    interaction_repo: Arc<IRepo>,
}

impl<IRepo> StatisticsService<IRepo>
where
    IRepo: InteractionRepository,
{
    pub fn new(interaction_repo: Arc<IRepo>) -> Self {
        StatisticsService { interaction_repo }
    }

    pub fn from_interactions(
        &self,
        views: &[View],
        readings: &[Reading],
        likes: &[Like],
        reviews: &[Review],
    ) -> Result<Statistics> {
        let unique_views = views.iter().fold(
            0u32,
            |acc, view| {
                if view.is_unique() {
                    acc + 1
                } else {
                    acc
                }
            },
        );

        let stars = reviews
            .iter()
            .fold(0u32, |acc, review| acc + review.stars().value() as u32);

        let stars = stars as f32 / reviews.len() as f32;

        Ok(Statistics::new(
            views.len() as u32,
            unique_views,
            readings.len() as u32,
            likes.len() as u32,
            reviews.len() as u32,
            stars,
        )?)
    }

    pub async fn get_history(
        &self,
        reader_id: Option<&ReaderId>,
        publication_id: Option<&PublicationId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Statistics> {
        let views = self
            .interaction_repo
            .find_views(
                reader_id.clone(),
                publication_id.clone(),
                from.clone(),
                to.clone(),
            )
            .await?;
        let readings = self
            .interaction_repo
            .find_readings(
                reader_id.clone(),
                publication_id.clone(),
                from.clone(),
                to.clone(),
            )
            .await?;
        let likes = self
            .interaction_repo
            .find_likes(
                reader_id.clone(),
                publication_id.clone(),
                from.clone(),
                to.clone(),
            )
            .await?;
        let reviews = self
            .interaction_repo
            .find_reviews(
                reader_id.clone(),
                publication_id.clone(),
                from.clone(),
                to.clone(),
            )
            .await?;

        self.from_interactions(&views, &readings, &likes, &reviews)
    }
}
