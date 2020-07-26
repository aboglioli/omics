use common::model::AggregateRoot;

use common::domain::user::UserId;

pub type DonationId = String;

pub struct Donation {
    base: AggregateRoot<DonationId>,
    issuer_id: UserId,
    receiver_id: UserId,
}
