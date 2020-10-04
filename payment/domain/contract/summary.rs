use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use common::error::Error;
use common::result::Result;
use publishing::domain::publication::Statistics;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    statistics: Statistics,
    total: f64,
    amount: f64,
    paid: bool,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

impl Summary {
    pub fn new(
        statistics: Statistics,
        total: f64,
        amount: f64,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Self> {
        if from >= to {
            return Err(Error::new("summary", "invalid_date_range"));
        }

        if amount > total || amount < 0.0 {
            return Err(Error::new("summary", "invalid_amount"));
        }

        if total < 0.0 {
            return Err(Error::new("summary", "invalid_total"));
        }

        Ok(Summary {
            statistics,
            total,
            amount,
            paid: false,
            from,
            to,
        })
    }

    pub fn statistics(&self) -> &Statistics {
        &self.statistics
    }

    pub fn total(&self) -> f64 {
        self.total
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn amount_percentage(&self) -> f64 {
        self.amount / self.total
    }

    pub fn from(&self) -> &DateTime<Utc> {
        &self.from
    }

    pub fn to(&self) -> &DateTime<Utc> {
        &self.to
    }

    pub fn is_paid(&self) -> bool {
        self.paid
    }

    pub fn pay(&mut self) -> Result<()> {
        if self.is_paid() {
            return Err(Error::new("summary", "already_paid"));
        }

        self.paid = true;

        Ok(())
    }
}
