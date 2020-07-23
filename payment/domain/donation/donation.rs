use common::model::{Entity, ID};

use crate::domain::user::{User, UserID};

pub type DonationID = String;

pub struct Donation {
    id: ID<DonationID>,
    issuer_id: UserID,
    receiver_id: UserID,
}

impl Entity<DonationID> for Donation {
    fn id(&self) -> &ID<DonationID> {
        &self.id
    }
}
