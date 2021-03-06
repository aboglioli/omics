mod repository;
mod status;
pub use repository::*;
pub use status::*;

use common::error::Error;
use common::model::{AggregateRoot, Events, StatusHistory, StringId};
use common::result::Result;
use identity::domain::user::UserId;
use publishing::domain::author::Author;
use publishing::domain::reader::Reader;
use shared::event::DonationEvent;

use crate::domain::payment::{Amount, Kind, Payment};

pub type DonationId = StringId;

#[derive(Debug, Clone)]
pub struct Donation {
    base: AggregateRoot<DonationId>,
    events: Events<DonationEvent>,
    author_id: UserId,
    reader_id: UserId,

    total: Amount,
    subtotal: Amount,
    author_percentage: f64,

    comment: String,

    reader_payment: Option<Payment>,
    author_charge: Option<Payment>,

    status_history: StatusHistory<Status>,
}

impl Donation {
    pub fn new<S: Into<String>>(
        id: DonationId,
        author: &Author,
        reader: &Reader,
        amount: Amount,
        comment: S,
        author_percentage: f64,
    ) -> Result<Self> {
        let subtotal = Amount::new(amount.value() * author_percentage)?;

        let mut donation = Donation {
            base: AggregateRoot::new(id),
            events: Events::new(),
            author_id: author.base().id().clone(),
            reader_id: reader.base().id().clone(),
            total: amount,
            subtotal,
            author_percentage,
            comment: comment.into(),
            reader_payment: None,
            author_charge: None,
            status_history: StatusHistory::new(Status::init()),
        };

        donation.events.record_event(DonationEvent::Created {
            id: donation.base().id().to_string(),
            author_id: donation.author_id().to_string(),
            reader_id: donation.reader_id().to_string(),
            total: donation.total().value(),
            subtotal: donation.subtotal().value(),
            author_percentage: donation.author_percentage(),
            comment: donation.comment().to_string(),
        });

        Ok(donation)
    }

    pub fn build(
        base: AggregateRoot<DonationId>,
        author_id: UserId,
        reader_id: UserId,
        total: Amount,
        subtotal: Amount,
        author_percentage: f64,
        comment: String,
        reader_payment: Option<Payment>,
        author_charge: Option<Payment>,
        status_history: StatusHistory<Status>,
    ) -> Self {
        Donation {
            base,
            events: Events::new(),
            author_id,
            reader_id,
            total,
            subtotal,
            author_percentage,
            comment,
            reader_payment,
            author_charge,
            status_history,
        }
    }

    pub fn base(&self) -> &AggregateRoot<DonationId> {
        &self.base
    }

    pub fn events(&self) -> &Events<DonationEvent> {
        &self.events
    }

    pub fn author_id(&self) -> &UserId {
        &self.author_id
    }

    pub fn reader_id(&self) -> &UserId {
        &self.reader_id
    }

    pub fn total(&self) -> &Amount {
        &self.total
    }

    pub fn subtotal(&self) -> &Amount {
        &self.subtotal
    }

    pub fn author_percentage(&self) -> f64 {
        self.author_percentage
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn reader_payment(&self) -> Option<&Payment> {
        self.reader_payment.as_ref()
    }

    pub fn author_charge(&self) -> Option<&Payment> {
        self.author_charge.as_ref()
    }

    pub fn status_history(&self) -> &StatusHistory<Status> {
        &self.status_history
    }

    pub fn is_paid(&self) -> bool {
        matches!(self.status_history.current(), Status::Paid) && self.reader_payment.is_some()
    }

    pub fn is_charged(&self) -> bool {
        matches!(self.status_history.current(), Status::Charged) && self.author_charge.is_some()
    }

    pub fn pay(&mut self) -> Result<Payment> {
        if self.is_paid() {
            return Err(Error::new("donation", "already_paid"));
        }

        let payment = Payment::new(Kind::Income, self.total().clone())?;

        let status = self.status_history.current().pay()?;
        self.status_history.add_status(status);

        self.reader_payment = Some(payment.clone());

        self.base.update();

        self.events.record_event(DonationEvent::Paid {
            id: self.base().id().to_string(),
            author_id: self.author_id().to_string(),
            reader_id: self.reader_id().to_string(),
            total: self.total().value(),
            subtotal: self.subtotal().value(),
            author_percentage: self.author_percentage(),
            comment: self.comment().to_string(),
        });

        Ok(payment)
    }

    pub fn charge(&mut self) -> Result<Payment> {
        if !self.is_paid() {
            return Err(Error::new("donation", "not_paid"));
        }

        if self.is_charged() {
            return Err(Error::new("donation", "already_charged"));
        }

        let payment = Payment::new(Kind::Outcome, self.subtotal().clone())?;

        let status = self.status_history.current().charge()?;
        self.status_history.add_status(status);

        self.author_charge = Some(payment.clone());

        self.base.update();

        self.events.record_event(DonationEvent::Charged {
            id: self.base().id().to_string(),
            author_id: self.author_id().to_string(),
            reader_id: self.reader_id().to_string(),
            total: self.total().value(),
            subtotal: self.subtotal().value(),
            author_percentage: self.author_percentage(),
            comment: self.comment().to_string(),
        });

        Ok(payment)
    }

    pub fn cancel(&mut self) -> Result<()> {
        let status = self.status_history.current().cancel()?;
        self.status_history.add_status(status);

        self.base.delete();

        self.events.record_event(DonationEvent::Cancelled {
            id: self.base().id().to_string(),
            author_id: self.author_id().to_string(),
            reader_id: self.reader_id().to_string(),
            total: self.total().value(),
            subtotal: self.subtotal().value(),
            author_percentage: self.author_percentage(),
            comment: self.comment().to_string(),
        });

        Ok(())
    }
}
