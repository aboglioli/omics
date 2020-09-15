use serde::Serialize;

use common::result::Result;
use shared::domain::user::UserRepository;

use crate::application::dtos::{ReaderDto, ReviewDto};
use crate::domain::interaction::InteractionRepository;
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderRepository;

#[derive(Serialize)]
pub struct GetReviewsResponse {
    pub reviews: Vec<ReviewDto>,
}

pub struct GetReviews<'a> {
    interaction_repo: &'a dyn InteractionRepository,
    reader_repo: &'a dyn ReaderRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> GetReviews<'a> {
    pub fn new(
        interaction_repo: &'a dyn InteractionRepository,
        reader_repo: &'a dyn ReaderRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        GetReviews {
            interaction_repo,
            reader_repo,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        _auth_id: Option<String>,
        publication_id: String,
    ) -> Result<GetReviewsResponse> {
        let reviews = self
            .interaction_repo
            .find_reviews(None, Some(&PublicationId::new(publication_id)?), None, None)
            .await?;

        let mut review_dtos = Vec::new();
        for review in reviews.iter() {
            let reader = self
                .reader_repo
                .find_by_id(review.base().id().reader_id())
                .await?;
            let user = self.user_repo.find_by_id(reader.base().id()).await?;
            review_dtos.push(ReviewDto::from(review).reader(ReaderDto::from(&user, &reader)));
        }

        Ok(GetReviewsResponse {
            reviews: review_dtos,
        })
    }
}
