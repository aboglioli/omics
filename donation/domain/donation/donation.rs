use common::model::{AggregateRoot, DefaultEvent};

use common::domain::user::UserId;

pub type DonationId = String;

pub struct Donation {
    base: AggregateRoot<DonationId, DefaultEvent>,
    issuer_id: UserId,
    receiver_id: UserId,
}
