use serde::Serialize;

use common::result::Result;

use crate::application::dtos::PlanDto;
use crate::domain::plan::PlanRepository;

#[derive(Serialize)]
pub struct GetAllResponse {
    plans: Vec<PlanDto>,
}

pub struct GetAll<'a> {
    plan_repo: &'a dyn PlanRepository,
}

impl<'a> GetAll<'a> {
    pub fn new(plan_repo: &'a dyn PlanRepository) -> Self {
        GetAll { plan_repo }
    }

    pub async fn exec(&self) -> Result<GetAllResponse> {
        let plans = self.plan_repo.find_all().await?;

        let mut plan_dtos = Vec::new();
        for plan in plans.into_iter() {
            plan_dtos.push(PlanDto::from(&plan));
        }

        Ok(GetAllResponse { plans: plan_dtos })
    }
}
