#[derive(Debug, Clone, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Rejected,
}
