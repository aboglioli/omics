use serde::Serialize;

use common::result::Result;

use crate::domain::interaction::InteractionRepository;
use crate::domain::publication::PublicationId;

#[derive(Serialize)]
pub struct ReviewDto {
    reader_id: String,
    stars: u8,
    comment: String,
}

#[derive(Serialize)]
pub struct ReviewsResponse {
    reviews: Vec<ReviewDto>,
}

pub struct Reviews<'a, IRepo> {
    interaction_repo: &'a IRepo,
}

impl<'a, IRepo> Reviews<'a, IRepo>
where
    IRepo: InteractionRepository,
{
    pub fn new(interaction_repo: &'a IRepo) -> Self {
        Reviews { interaction_repo }
    }

    pub async fn exec(&self, publication_id: String) -> Result<ReviewsResponse> {
        let reviews = self
            .interaction_repo
            .find_reviews(None, Some(&PublicationId::new(publication_id)?), None, None)
            .await?;

        let mut review_dtos = Vec::new();
        for review in reviews {
            review_dtos.push(ReviewDto {
                reader_id: review.base().reader_id().value().to_owned(),
                stars: review.stars().value(),
                comment: review.comment().value().to_owned(),
            });
        }

        Ok(ReviewsResponse {
            reviews: review_dtos,
        })
    }
}
