use serde::Serialize;

use common::result::Result;

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
}

impl<'a> GetReviews<'a> {
    pub fn new(
        interaction_repo: &'a dyn InteractionRepository,
        reader_repo: &'a dyn ReaderRepository,
    ) -> Self {
        GetReviews {
            interaction_repo,
            reader_repo,
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
            review_dtos.push(ReviewDto::from(review).reader(ReaderDto::from(&reader)));
        }

        Ok(GetReviewsResponse {
            reviews: review_dtos,
        })
    }
}
