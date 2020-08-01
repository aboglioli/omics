pub struct DonateCommand {
    issuer_id: String,
    receiver_id: String,
}

pub struct DonateToAuthor {
    user_repository: Arc<dyn UserRepository>,
    donation_repository: Arc<dyn DonationRepository>,
}

impl DonateToAuthor {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        donation_repository: Arc<dyn DonationRepository>,
    ) -> Self {
        DonateToAuthor {
            user_repository,
            donation_repository,
        }
    }

    pub fn execute(&self, cmd: DonateCommand) -> Result<()> {
        let _ = Donation::new(
            self.donation_repository.next_id()?,
            UserId::from(cmd.issuer_id),
            UserId::from(cmd.receiver_id),
        );

        Ok(())
    }
}
