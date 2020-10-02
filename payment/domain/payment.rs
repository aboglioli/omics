mod amount;
mod kind;
pub use amount::*;
pub use kind::*;

use chrono::{DateTime, Duration, Utc};

use common::result::Result;

#[derive(Debug, Clone)]
pub struct Payment {
    kind: Kind,
    amount: Amount,
    date: DateTime<Utc>,
}

impl Payment {
    pub fn new(kind: Kind, amount: Amount) -> Result<Self> {
        Ok(Payment {
            kind,
            amount,
            date: Utc::now(),
        })
    }

    pub fn build(kind: Kind, amount: Amount, date: DateTime<Utc>) -> Self {
        Payment { kind, amount, date }
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn is_current(&self, days: i64) -> bool {
        self.date + Duration::days(days) > Utc::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_current() {
        let payment = Payment::build(Kind::Income, Amount::new(45.0).unwrap(), Utc::now());
        assert!(payment.is_current(30));

        let payment = Payment::build(
            Kind::Income,
            Amount::new(45.0).unwrap(),
            Utc::now() - Duration::days(15),
        );
        assert!(payment.is_current(30));

        let payment = Payment::build(
            Kind::Income,
            Amount::new(45.0).unwrap(),
            Utc::now() - Duration::days(45),
        );
        assert!(!payment.is_current(30));
    }
}
