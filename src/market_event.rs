use chrono::{DateTime, Utc};
use serde::Deserialize;

// market event "class"
#[derive(Debug, Clone, Deserialize)]
pub struct MarketEvent {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub price: f64,
    pub size: u32,
}
