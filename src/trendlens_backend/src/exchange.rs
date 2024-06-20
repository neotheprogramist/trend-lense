use candid::CandidType;
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Deserialize, Debug, CandidType)]
pub enum Exchange {
    Okx,
}

#[derive(Deserialize, CandidType, Serialize, Clone)]
pub struct Candle {
    pub timestamp: u64,
    pub open_price: f64,
    pub highest_price: f64,
    pub lowest_price: f64,
    pub close_price: f64,
}
