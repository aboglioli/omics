use async_trait::async_trait;
use uuid::Uuid;

use common::result::Result;

use crate::domain::plan::{Plan, PlanId};

#[async_trait]
pub trait PlanRepository: Sync + Send {
    async fn next_id(&self) -> Result<PlanId> {
        PlanId::new(Uuid::new_v4().to_string())
    }

    async fn find_all(&self) -> Result<Vec<Plan>>;
    async fn find_by_id(&self, id: &PlanId) -> Result<Plan>;

    async fn save(&self, plan: &mut Plan) -> Result<()>;

    async fn delete(&self, id: &PlanId) -> Result<()>;
}
