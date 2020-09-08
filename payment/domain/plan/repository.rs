use async_trait::async_trait;

use common::result::Result;

use crate::domain::plan::{Plan, PlanId};

#[async_trait]
pub trait PlanRepository {
    async fn find_by_id(&self, id: &PlanId) -> Result<Plan>;

    async fn save(&self, plan: &mut Plan) -> Result<()>;
}
