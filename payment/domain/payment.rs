mod amount;
mod kind;
pub use amount::*;
pub use kind::*;

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use common::result::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    kind: Kind,
    #[serde(flatten)]
    amount: Amount,
    datetime: DateTime<Utc>,
}

impl Payment {
    pub fn new(kind: Kind, amount: Amount) -> Result<Self> {
        Ok(Payment {
            kind,
            amount,
            datetime: Utc::now(),
        })
    }

    pub fn build(kind: Kind, amount: Amount, datetime: DateTime<Utc>) -> Self {
        Payment {
            kind,
            amount,
            datetime,
        }
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn datetime(&self) -> &DateTime<Utc> {
        &self.datetime
    }

    pub fn is_current(&self, days: i64) -> bool {
        self.datetime + Duration::days(days) > Utc::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    use serde_json::json;

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

    #[test]
    fn serialize() {
        let payment = Payment::build(
            Kind::Income,
            Amount::new(48.57).unwrap(),
            DateTime::from_str("2020-05-05T18:21:00Z").unwrap(),
        );
        let payment = serde_json::to_value(payment).unwrap();

        let expected = json!({
            "kind": "income",
            "amount": 48.57,
            "datetime": "2020-05-05T18:21:00Z",
        });

        assert_eq!(payment, expected);
    }

    #[test]
    fn deserialize() {
        let payment: Payment = serde_json::from_str(
            r#"{
            "kind": "outcome",
            "amount": 56.14,
            "datetime": "2020-05-05T18:21:00Z"
        }"#,
        )
        .unwrap();
        assert_eq!(payment.kind().to_string(), "outcome");
        assert_eq!(payment.amount().value(), 56.14);
        assert_eq!(payment.datetime().to_rfc3339(), "2020-05-05T18:21:00+00:00");
    }
}
