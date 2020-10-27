use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use common::counter::Counter;
use common::result::Result;
use identity::domain::user::User;
use payment::domain::contract::Contract;
use payment::domain::donation::Donation;
use payment::domain::subscription::Subscription;
use publishing::domain::publication::Publication;

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
    pub income: f64,
    pub outcome: f64,
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
                by_status.inc("active");
            } else if !user.is_validated() {
                by_status.inc("not-validated");
            } else {
                by_status.inc("inative");
            }

            if let Some(person) = user.person() {
                if let Some(gender) = person.gender() {
                    by_gender.inc(gender.to_string());
                } else {
                    by_gender.inc("unknown");
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
                    by_age.inc("unknown");
                }
            } else {
                by_gender.inc("unknown");
                by_age.inc("unknown");
            }
        }

        self.users = Some(Users {
            total: users.len(),
            by_status: by_status.into(),
            by_gender: by_gender.into(),
            by_age: by_age.into(),
        });
    }

    pub fn map_publications(&mut self, publications: &[Publication]) {
        let mut by_category = Counter::new();
        let mut by_contract = Counter::new();
        let mut by_status = Counter::new();
        let mut by_pages = Counter::new();

        for publication in publications.iter() {
            by_category.inc(publication.header().category_id().to_string());
            by_contract.inc(if publication.has_contract() {
                "with-contract"
            } else {
                "without-contract"
            });
            by_status.inc(publication.status_history().current().to_string());
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
            by_status.inc(subscription.status_history().current().to_string());

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
            by_status.inc(contract.status_history().current().to_string());

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
            by_status.inc(donation.status_history().current().to_string());

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

    pub fn map_payments(&mut self, subscriptions: &[Subscription], contracts: &[Contract]) {
        let mut income = 0.0;
        let mut outcome = 0.0;

        for subscription in subscriptions.iter() {
            for payment in subscription.payments().iter() {
                if payment.datetime() >= &self.from && payment.datetime() <= &self.to {
                    income += payment.amount().value();
                }
            }
        }

        for contract in contracts.iter() {
            for payment in contract.payments().iter() {
                if payment.datetime() >= &self.from && payment.datetime() <= &self.to {
                    outcome += payment.amount().value();
                }
            }
        }

        self.payments = Some(Payments { income, outcome });
    }
}
