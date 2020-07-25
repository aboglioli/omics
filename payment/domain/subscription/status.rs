#[derive(Debug, Clone, PartialEq)]
pub enum SubscriptionStatus {
    Active,
    WaitingPayment,
    Deactivated,
}
