use common::model::AggregateRoot;

use crate::domain::user::{User, UserId};

pub type DonationId = String;

pub struct Donation {
    base: AggregateRoot<DonationId>,
    issuer_id: UserId,
    receiver_id: UserId,
}
