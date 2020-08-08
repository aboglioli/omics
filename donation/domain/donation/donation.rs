use common::event::Event;
use common::model::AggregateRoot;

use identity::domain::user::UserId;

pub type DonationId = String;

pub struct Donation {
    base: AggregateRoot<DonationId, Event>,
    issuer_id: UserId,
    receiver_id: UserId,
}

impl Donation {
    pub fn base(&self) -> &AggregateRoot<DonationId, Event> {
        &self.base
    }

    pub fn issuer_id(&self) -> &UserId {
        &self.issuer_id
    }

    pub fn receiver_id(&self) -> &UserId {
        &self.receiver_id
    }
}
