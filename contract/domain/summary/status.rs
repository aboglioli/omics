#[derive(Debug, Clone, PartialEq)]
pub enum SummaryStatus {
    Open,
    ReadyToPay,
    Paid,
    Cancelled,
}
