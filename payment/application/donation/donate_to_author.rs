pub struct DonateCommand {
    issuer_id: String,
    receiver_id: String,
}

pub struct DonateToAuthor {
    user_repository: Rc<dyn UserRepository>,
    donation_repository: Rc<dyn DonationRepository>,
}

impl DonateToAuthor {
    pub fn new(
        user_repository: Rc<dyn UserRepository>,
        donation_repository: Rc<dyn DonationRepository>,
    ) -> Self {
        DonateToAuthor {
            user_repository,
            donation_repository,
        }
    }

    pub fn execute(&self, cmd: DonateCommand) -> Result<(), Error> {
        let _ = Donation::new(
            self.donation_repository.next_id()?,
            UserId::from(cmd.issuer_id),
            UserId::from(cmd.receiver_id),
        );

        Ok(())
    }
}
