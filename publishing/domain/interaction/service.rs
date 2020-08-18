use std::sync::Arc;

use common::error::Error;
use common::result::Result;

use crate::domain::interaction::{
    Comment, InteractionRepository, Like, Reading, Review, Stars, View,
};
use crate::domain::publication::Publication;
use crate::domain::reader::Reader;

pub struct InteractionService {
    interaction_repo: Arc<dyn InteractionRepository>,
}

impl InteractionService {
    pub fn new(interaction_repo: Arc<dyn InteractionRepository>) -> Self {
        InteractionService { interaction_repo }
    }

    pub async fn add_view(&self, reader: &Reader, publication: &mut Publication) -> Result<View> {
        let views_res = self
            .interaction_repo
            .find_views(
                Some(&reader.base().id()),
                Some(&publication.base().id()),
                None,
                None,
            )
            .await;
        let unique = if let Ok(views) = views_res {
            views.len() == 0
        } else {
            true
        };

        let mut view = publication.view(reader, unique)?;
        self.interaction_repo.save_view(&mut view).await?;

        Ok(view)
    }

    pub async fn add_reading(
        &self,
        reader: &Reader,
        publication: &mut Publication,
    ) -> Result<Reading> {
        let mut reading = publication.read(reader)?;
        self.interaction_repo.save_reading(&mut reading).await?;

        Ok(reading)
    }

    pub async fn add_like(&self, reader: &Reader, publication: &mut Publication) -> Result<Like> {
        let likes_res = self
            .interaction_repo
            .find_likes(
                Some(&reader.base().id()),
                Some(&publication.base().id()),
                None,
                None,
            )
            .await;
        if let Ok(likes) = likes_res {
            if likes.len() > 0 {
                return Err(Error::new("like", "already_liked"));
            }
        }

        let mut like = publication.like(reader)?;
        self.interaction_repo.save_like(&mut like).await?;

        Ok(like)
    }

    pub async fn delete_like(&self, reader: &Reader, publication: &mut Publication) -> Result<()> {
        let likes = self
            .interaction_repo
            .find_likes(
                Some(&reader.base().id()),
                Some(&publication.base().id()),
                None,
                None,
            )
            .await?;
        if likes.len() == 0 {
            return Err(Error::new("like", "not_liked"));
        }

        publication.unlike(reader)?;

        self.interaction_repo
            .delete_like(&reader.base().id(), &publication.base().id())
            .await?;

        Ok(())
    }

    pub async fn add_review(
        &self,
        reader: &Reader,
        publication: &mut Publication,
        stars: Stars,
        comment: Comment,
    ) -> Result<Review> {
        let reviews_res = self
            .interaction_repo
            .find_reviews(
                Some(&reader.base().id()),
                Some(&publication.base().id()),
                None,
                None,
            )
            .await;
        if let Ok(reviews) = reviews_res {
            if reviews.len() > 0 {
                return Err(Error::new("review", "existing"));
            }
        }

        let mut review = publication.review(reader, stars, comment)?;
        self.interaction_repo.save_review(&mut review).await?;

        Ok(review)
    }

    pub async fn delete_review(
        &self,
        reader: &Reader,
        publication: &mut Publication,
    ) -> Result<()> {
        let mut reviews = self
            .interaction_repo
            .find_reviews(
                Some(&reader.base().id()),
                Some(&publication.base().id()),
                None,
                None,
            )
            .await?;
        if reviews.len() == 0 {
            return Err(Error::new("review", "not_reviewed"));
        }

        let review = reviews.remove(0);

        publication.delete_review(reader, review.stars())?;

        self.interaction_repo
            .delete_review(&reader.base().id(), &publication.base().id())
            .await?;

        Ok(())
    }
}
