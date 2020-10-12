use serde::Serialize;

use common::model::StatusItem;
use identity::application::dtos::UserDto;
use publishing::application::dtos::{PublicationDto, StatisticsDto};

use crate::domain::contract::{Contract, Status as ContractStatus, Summary};
use crate::domain::payment::Payment;
use crate::domain::plan::Plan;
use crate::domain::subscription::{Status as SubscriptionStatus, Subscription, SubscriptionPlan};

#[derive(Serialize)]
pub struct PlanDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
}

impl PlanDto {
    pub fn from(plan: &Plan) -> Self {
        PlanDto {
            id: plan.base().id().to_string(),
            name: plan.name().to_string(),
            description: plan.description().to_string(),
            price: plan.price().value(),
        }
    }
}

#[derive(Serialize)]
pub struct PaymentDto {
    pub kind: String,
    pub amount: f64,
    pub datetime: String,
}

impl PaymentDto {
    pub fn from(payment: &Payment) -> Self {
        PaymentDto {
            kind: payment.kind().to_string(),
            amount: payment.amount().value(),
            datetime: payment.datetime().to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct SubscriptionPlanDto {
    pub id: String,
    pub price: f64,
    pub assigned_at: String,
}

impl SubscriptionPlanDto {
    pub fn from(plan: &SubscriptionPlan) -> Self {
        SubscriptionPlanDto {
            id: plan.plan_id().to_string(),
            price: plan.price(),
            assigned_at: plan.assigned_at().to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct SubscriptionStatusDto {
    pub status: String,
    pub changed_at: String,
}

impl SubscriptionStatusDto {
    pub fn from(status_item: &StatusItem<SubscriptionStatus>) -> Self {
        let status = status_item.status();

        SubscriptionStatusDto {
            status: status.to_string(),
            changed_at: status_item.datetime().to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct SubscriptionDto {
    pub id: String,
    pub user_id: Option<String>,
    pub user: Option<UserDto>,
    pub plan: SubscriptionPlanDto,
    pub payments: Vec<PaymentDto>,
    pub status: SubscriptionStatusDto,
}

impl SubscriptionDto {
    pub fn from(subscription: &Subscription) -> Self {
        SubscriptionDto {
            id: subscription.base().id().to_string(),
            user_id: Some(subscription.user_id().to_string()),
            user: None,
            plan: SubscriptionPlanDto::from(subscription.plan()),
            payments: subscription
                .payments()
                .iter()
                .map(PaymentDto::from)
                .collect(),
            status: SubscriptionStatusDto::from(subscription.status_history().current_item()),
        }
    }

    pub fn user(mut self, user: UserDto) -> Self {
        self.user_id = None;
        self.user = Some(user);
        self
    }
}

#[derive(Serialize)]
pub struct SummaryDto {
    pub statistics: StatisticsDto,
    pub total: f64,
    pub amount: f64,
    pub paid: bool,
    pub from: String,
    pub to: String,
}

impl SummaryDto {
    pub fn from(summary: &Summary) -> Self {
        SummaryDto {
            statistics: StatisticsDto::from(summary.statistics()),
            total: summary.total(),
            amount: summary.amount(),
            paid: summary.is_paid(),
            from: summary.from().to_rfc3339(),
            to: summary.to().to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct ContractStatusDto {
    pub status: String,
    pub changed_at: String,
    pub changed_by: Option<String>,
}

impl ContractStatusDto {
    pub fn from(status_item: &StatusItem<ContractStatus>) -> Self {
        let status = status_item.status();

        let mut dto = ContractStatusDto {
            status: status.to_string(),
            changed_at: status_item.datetime().to_rfc3339(),
            changed_by: None,
        };

        match status {
            ContractStatus::Approved { admin_id } | ContractStatus::Rejected { admin_id } => {
                dto.changed_by = Some(admin_id.to_string());
            }
            _ => {}
        }

        dto
    }
}

#[derive(Serialize)]
pub struct ContractDto {
    pub id: String,
    pub publication_id: Option<String>,
    pub publication: Option<PublicationDto>,
    pub summaries: Vec<SummaryDto>,
    pub payments: Vec<PaymentDto>,
    pub status: ContractStatusDto,
}

impl ContractDto {
    pub fn from(contract: &Contract) -> Self {
        ContractDto {
            id: contract.base().id().to_string(),
            publication_id: Some(contract.publication_id().to_string()),
            publication: None,
            summaries: contract.summaries().iter().map(SummaryDto::from).collect(),
            payments: contract.payments().iter().map(PaymentDto::from).collect(),
            status: ContractStatusDto::from(contract.status_history().current_item()),
        }
    }

    pub fn publication(mut self, publication: PublicationDto) -> Self {
        self.publication_id = None;
        self.publication = Some(publication);
        self
    }
}
