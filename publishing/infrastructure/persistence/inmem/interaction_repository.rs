use async_trait::async_trait;
use tokio::sync::Mutex;

use common::result::Result;

use crate::domain::interaction::{FindOpts, Interaction, InteractionRepository};

pub struct InMemInteractionRepository {
    interactions: Mutex<Vec<Interaction>>,
}

impl InMemInteractionRepository {
    pub fn new() -> Self {
        InMemInteractionRepository {
            interactions: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl InteractionRepository for InMemInteractionRepository {
    async fn find(&self, opts: &FindOpts<'_>) -> Result<Vec<Interaction>> {
        Ok(self
            .interactions
            .lock()
            .await
            .iter()
            .filter(|interaction| {
                if let Some(publication_id) = opts.publication_id {
                    if interaction.publication_id() != publication_id {
                        return false;
                    }
                }

                if let Some(reader_id) = opts.reader_id {
                    if interaction.reader_id() != reader_id {
                        return false;
                    }
                }

                if let Some(_kind) = opts.kind {
                    if !matches!(interaction.kind(), _kind) {
                        return false;
                    }
                }

                return true;
            })
            .cloned()
            .collect())
    }

    async fn save(&self, interaction: &mut Interaction) -> Result<()> {
        self.interactions.lock().await.push(interaction.clone());

        Ok(())
    }
}
