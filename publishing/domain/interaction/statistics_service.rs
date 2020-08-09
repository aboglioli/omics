use common::result::Result;

use crate::domain::interaction::{FindOpts, InteractionRepository, Statistics};
use crate::domain::publication::PublicationId;

pub struct StatisticsService<'a, IRepo> {
    interaction_repo: &'a IRepo,
}

impl<'a, IRepo> StatisticsService<'a, IRepo>
where
    IRepo: InteractionRepository,
{
    pub fn new(interaction_repo: &'a IRepo) -> Self {
        StatisticsService { interaction_repo }
    }

    pub async fn get_all_statistics(&self, publication_id: &PublicationId) -> Result<Statistics> {
        let find_opts = FindOpts {
            publication_id: Some(publication_id),
            reader_id: None,
            from: None,
            to: None,
        };

        let _views = self.interaction_repo.find_views(&find_opts).await?;
        let _readings = self.interaction_repo.find_readings(&find_opts).await?;
        let _likes = self.interaction_repo.find_likes(&find_opts).await?;
        let _reviews = self.interaction_repo.find_reviews(&find_opts).await?;

        Ok(Statistics::default())
    }
}
