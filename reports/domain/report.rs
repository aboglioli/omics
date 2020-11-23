use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use common::counter::Counter;
use common::result::Result;
use identity::domain::user::{Gender, User};
use payment::domain::contract::{Contract, Status as ContractStatus};
use payment::domain::donation::{Donation, Status as DonationStatus};
use payment::domain::subscription::{Status as SubscriptionStatus, Subscription};
use publishing::domain::publication::{Publication, Status as PublicationStatus};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Users {
    pub total: usize,
    pub by_status: HashMap<String, usize>,
    pub by_gender: HashMap<String, usize>,
    pub by_age: HashMap<String, usize>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Authors {
    pub total: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Publications {
    pub total: usize,
    pub by_category: HashMap<String, usize>,
    pub by_contract: HashMap<String, usize>,
    pub by_status: HashMap<String, usize>,
    pub by_pages: HashMap<String, usize>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Subscriptions {
    pub total: usize,
    pub by_payment: HashMap<String, usize>,
    pub by_status: HashMap<String, usize>,
    pub by_amount: HashMap<String, usize>,
    pub amount: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Contracts {
    pub total: usize,
    pub by_summary: HashMap<String, usize>,
    pub by_payment: HashMap<String, usize>,
    pub by_status: HashMap<String, usize>,
    pub by_amount: HashMap<String, usize>,
    pub amount: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Donations {
    pub total: usize,
    pub by_status: HashMap<String, usize>,
    pub by_amount: HashMap<String, usize>,
    pub amount: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Payments {
    pub total_income: f64,
    pub subscription_income: f64,
    pub donation_income: f64,
    pub total_outcome: f64,
    pub contract_outcome: f64,
    pub donation_outcome: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub users: Option<Users>,
    pub authors: Option<Authors>,
    pub publications: Option<Publications>,
    pub subscriptions: Option<Subscriptions>,
    pub contracts: Option<Contracts>,
    pub donations: Option<Donations>,
    pub payments: Option<Payments>,

    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

impl Report {
    pub fn new(from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Self> {
        Ok(Report {
            users: None,
            authors: None,
            publications: None,
            subscriptions: None,
            contracts: None,
            donations: None,
            payments: None,

            from,
            to,
        })
    }

    pub fn map_users(&mut self, users: &[User]) {
        let mut by_status = Counter::new();
        let mut by_gender = Counter::new();
        let mut by_age = Counter::new();

        for user in users.iter() {
            if user.is_active() {
                by_status.inc("Activos");
            } else if !user.is_validated() {
                by_status.inc("No validados");
            } else {
                by_status.inc("inative");
            }

            if let Some(person) = user.person() {
                if let Some(gender) = person.gender() {
                    by_gender.inc(match gender {
                        Gender::Male => "Masculino",
                        Gender::Female => "Femenino",
                        Gender::Other => "Otro",
                    });
                } else {
                    by_gender.inc("Desconocido");
                }

                if let Some(birthdate) = person.birthdate() {
                    let d = Utc::now() - birthdate.date().clone();
                    let age = d.num_days() / 365;

                    if age < 14 {
                        by_age.inc("0-14");
                    } else if age < 22 {
                        by_age.inc("14-22");
                    } else if age < 30 {
                        by_age.inc("22-30");
                    } else if age < 40 {
                        by_age.inc("30-40");
                    } else if age < 50 {
                        by_age.inc("40-50");
                    } else if age < 60 {
                        by_age.inc("50-60");
                    } else {
                        by_age.inc("+60");
                    }
                } else {
                    by_age.inc("Desconocida");
                }
            } else {
                by_gender.inc("Desconocido");
                by_age.inc("Desconocido");
            }
        }

        self.users = Some(Users {
            total: users.len(),
            by_status: by_status.into(),
            by_gender: by_gender.into(),
            by_age: by_age.into(),
        });
    }

    pub fn map_publications(
        &mut self,
        publications: &[Publication],
        categories: HashMap<String, String>,
    ) {
        let mut by_category = Counter::new();
        let mut by_contract = Counter::new();
        let mut by_status = Counter::new();
        let mut by_pages = Counter::new();

        for publication in publications.iter() {
            if let Some(category_name) = categories.get(publication.header().category_id().value())
            {
                by_category.inc(category_name);
            }

            by_contract.inc(if publication.has_contract() {
                "Con contrato"
            } else {
                "Sin contrato"
            });

            by_status.inc(match publication.status_history().current() {
                PublicationStatus::Draft => "Borrador",
                PublicationStatus::WaitingApproval => "Esperando aprobación",
                PublicationStatus::Published { .. } => "Publicada",
                PublicationStatus::Rejected { .. } => "Rechazada",
            });

            by_pages.inc(publication.pages().len().to_string());
        }

        self.publications = Some(Publications {
            total: publications.len(),
            by_category: by_category.into(),
            by_contract: by_contract.into(),
            by_status: by_status.into(),
            by_pages: by_pages.into(),
        });
    }

    pub fn map_subscriptions(&mut self, subscriptions: &[Subscription]) {
        let mut by_payment = Counter::new();
        let mut by_status = Counter::new();
        let mut by_amount = Counter::new();
        let mut amount = 0.0;

        for subscription in subscriptions.iter() {
            by_payment.inc(subscription.payments().len().to_string());
            by_status.inc(match subscription.status_history().current() {
                SubscriptionStatus::WaitingForPayment => "Esperando pago",
                SubscriptionStatus::Active => "Activa",
                SubscriptionStatus::Inactive => "Inactive",
            });

            let mut s_amount = 0.0;

            for payment in subscription.payments() {
                s_amount += payment.amount().value();
            }

            if s_amount < 100.0 {
                by_amount.inc("0-100");
            } else if s_amount < 200.0 {
                by_amount.inc("100-200");
            } else if s_amount < 300.0 {
                by_amount.inc("200-300");
            } else if s_amount < 400.0 {
                by_amount.inc("300-400");
            } else if s_amount < 500.0 {
                by_amount.inc("400-500");
            } else {
                by_amount.inc("+500");
            }

            amount += s_amount;
        }

        self.subscriptions = Some(Subscriptions {
            total: subscriptions.len(),
            by_payment: by_payment.into(),
            by_status: by_status.into(),
            by_amount: by_amount.into(),
            amount,
        });
    }

    pub fn map_contracts(&mut self, contracts: &[Contract]) {
        let mut by_summary = Counter::new();
        let mut by_payment = Counter::new();
        let mut by_status = Counter::new();
        let mut by_amount = Counter::new();
        let mut amount = 0.0;

        for contract in contracts.iter() {
            by_summary.inc(contract.summaries().len().to_string());
            by_payment.inc(contract.payments().len().to_string());
            by_status.inc(match contract.status_history().current() {
                ContractStatus::Requested => "Requerido",
                ContractStatus::Approved { .. } => "Aprobado",
                ContractStatus::Rejected { .. } => "Rechazado",
                ContractStatus::Cancelled => "Cancelado",
            });

            let mut c_amount = 0.0;

            for payment in contract.payments() {
                c_amount += payment.amount().value();
            }

            if c_amount < 300.0 {
                by_amount.inc("0-300");
            } else if c_amount < 600.0 {
                by_amount.inc("300-600");
            } else if c_amount < 900.0 {
                by_amount.inc("600-900");
            } else if c_amount < 1200.0 {
                by_amount.inc("900-1200");
            } else if c_amount < 1500.0 {
                by_amount.inc("1200-1500");
            } else {
                by_amount.inc("+1500");
            }

            amount += c_amount;
        }

        self.contracts = Some(Contracts {
            total: contracts.len(),
            by_summary: by_summary.into(),
            by_payment: by_payment.into(),
            by_status: by_status.into(),
            by_amount: by_amount.into(),
            amount,
        });
    }

    pub fn map_donations(&mut self, donations: &[Donation]) {
        let mut by_status = Counter::new();
        let mut by_amount = Counter::new();
        let mut amount = 0.0;

        for donation in donations.iter() {
            by_status.inc(match donation.status_history().current() {
                DonationStatus::WaitingForPayment => "Esperando pago",
                DonationStatus::Paid => "Pagada",
                DonationStatus::Charged => "Cobrada",
                DonationStatus::Cancelled => "Cancelada",
            });

            let d_amount = donation.total().value();

            if d_amount < 50.0 {
                by_amount.inc("0-50");
            } else if d_amount < 100.0 {
                by_amount.inc("50-100");
            } else if d_amount < 150.0 {
                by_amount.inc("100-150");
            } else if d_amount < 200.0 {
                by_amount.inc("150-200");
            } else if d_amount < 250.0 {
                by_amount.inc("200-250");
            } else {
                by_amount.inc("+250");
            }

            amount += d_amount;
        }

        self.donations = Some(Donations {
            total: donations.len(),
            by_status: by_status.into(),
            by_amount: by_amount.into(),
            amount,
        });
    }

    pub fn map_payments(
        &mut self,
        subscriptions: &[Subscription],
        contracts: &[Contract],
        donations: &[Donation],
    ) {
        let mut subscription_income = 0.0;
        let mut donation_income = 0.0;
        let mut contract_outcome = 0.0;
        let mut donation_outcome = 0.0;

        for subscription in subscriptions.iter() {
            for payment in subscription.payments().iter() {
                if payment.datetime() >= &self.from && payment.datetime() <= &self.to {
                    subscription_income += payment.amount().value();
                }
            }
        }

        for contract in contracts.iter() {
            for payment in contract.payments().iter() {
                if payment.datetime() >= &self.from && payment.datetime() <= &self.to {
                    contract_outcome += payment.amount().value();
                }
            }
        }

        for donation in donations.iter() {
            donation_income += donation.total().value() - donation.subtotal().value();
            donation_outcome += donation.subtotal().value();
        }

        self.payments = Some(Payments {
            total_income: subscription_income + donation_income,
            subscription_income,
            donation_income,
            total_outcome: contract_outcome + donation_outcome,
            contract_outcome,
            donation_outcome,
        });
    }
}
