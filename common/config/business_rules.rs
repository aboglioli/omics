use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessRules {
    // Donation
    pub minimum_donation_amount: f64,
    pub donation_percentage_retention: f64,

    // Contract
    pub days_to_generate_summaries: u64,
    pub minimum_views_percentage_to_require_contract: f64,
    pub subscription_percentage_retention: f64,
    pub minimum_charge_amount: f64,
}
