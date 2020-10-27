use serde::Serialize;

use common::model::StatusItem;
use identity::application::dtos::UserDto;
use publishing::application::dtos::{AuthorDto, PublicationDto, ReaderDto, StatisticsDto};

use crate::domain::contract::{Contract, Status as ContractStatus, Summary};
use crate::domain::donation::{Donation, Status as DonationStatus};
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
            ContractStatus::Approved {
                admin_id: Some(admin_id),
            }
            | ContractStatus::Rejected {
                admin_id: Some(admin_id),
            } => {
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

#[derive(Serialize)]
pub struct DonationStatusDto {
    pub status: String,
    pub changed_at: String,
}

impl DonationStatusDto {
    pub fn from(status_item: &StatusItem<DonationStatus>) -> Self {
        let status = status_item.status();

        DonationStatusDto {
            status: status.to_string(),
            changed_at: status_item.datetime().to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct DonationDto {
    pub id: String,
    pub author_id: Option<String>,
    pub author: Option<AuthorDto>,
    pub reader_id: Option<String>,
    pub reader: Option<ReaderDto>,
    pub total: f64,
    pub subtotal: f64,
    pub author_percentage: f64,
    pub comment: String,
    pub reader_payment: Option<PaymentDto>,
    pub author_charge: Option<PaymentDto>,
    pub status: DonationStatusDto,
}

impl DonationDto {
    pub fn from(donation: &Donation) -> Self {
        DonationDto {
            id: donation.base().id().to_string(),
            author_id: Some(donation.author_id().to_string()),
            author: None,
            reader_id: Some(donation.reader_id().to_string()),
            reader: None,
            total: donation.total().value(),
            subtotal: donation.subtotal().value(),
            author_percentage: donation.author_percentage(),
            comment: donation.comment().to_string(),
            reader_payment: donation.reader_payment().map(PaymentDto::from),
            author_charge: donation.author_charge().map(PaymentDto::from),
            status: DonationStatusDto::from(donation.status_history().current_item()),
        }
    }

    pub fn author(mut self, author: AuthorDto) -> Self {
        self.author_id = None;
        self.author = Some(author);
        self
    }

    pub fn reader(mut self, reader: ReaderDto) -> Self {
        self.reader_id = None;
        self.reader = Some(reader);
        self
    }
}
