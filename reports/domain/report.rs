use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use common::result::Result;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Users {
    pub total: usize,
    pub new: usize,
    pub by_gender: HashMap<String, usize>,
    pub by_age: HashMap<String, usize>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Authors {
    pub total: usize,
    pub new: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Publications {
    pub total: usize,
    pub new: usize,
    pub by_category: HashMap<String, usize>,
    pub by_preferences: HashMap<String, usize>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Subscriptions {
    pub total: usize,
    pub new: usize,
    pub amount: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Contracts {
    pub total: usize,
    pub new: usize,
    pub amount: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Payments {
    pub income: f64,
    pub outcome: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub users: Option<Users>,
    pub authors: Option<Authors>,
    pub publications: Option<Publications>,
    pub subscriptions: Option<Subscriptions>,
    pub contracts: Option<Contracts>,
    pub payments: Option<Payments>,

    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

impl Report {
    pub fn new(from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Self> {
        Ok(Report {
            users: None,
            authors: None,
            publications: None,
            subscriptions: None,
            contracts: None,
            payments: None,

            from,
            to,
        })
    }
}
