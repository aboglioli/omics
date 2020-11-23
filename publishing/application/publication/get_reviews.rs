use serde::Serialize;

use common::error::Error;
use common::result::Result;
use identity::UserIdAndRole;

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
        user_id_and_role: Option<UserIdAndRole>,
        publication_id: String,
    ) -> Result<GetReviewsResponse> {
        if let Some((_, auth_role)) = user_id_and_role {
            if !auth_role.can("get_publication_reviews") {
                return Err(Error::unauthorized());
            }
        }

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
