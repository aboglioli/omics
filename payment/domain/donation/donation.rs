use common::model::AggregateRoot;

use crate::domain::user::{User, UserID};

pub type DonationID = String;

pub struct Donation {
    base: AggregateRoot<DonationID>,
    issuer_id: UserID,
    receiver_id: UserID,
}
