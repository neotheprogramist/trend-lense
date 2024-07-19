use super::{okx::api::InstrumentType, ExchangeErrors};
use crate::pair::Pair;
use candid::CandidType;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait ApiResponseWrapper<R: DeserializeOwned>: DeserializeOwned {
    fn extract_response(self) -> Result<R, ExchangeErrors>;
}

#[derive(Deserialize, Debug, Clone, CandidType, Serialize)]
pub struct Instrument {
    pub instrument_id: Pair,
    pub instrument_type: InstrumentType,
}

#[derive(Deserialize, Debug, Clone, CandidType, Serialize)]
pub struct OrderData {
    pub code: String,
}

#[derive(Deserialize, Debug, Clone, CandidType, Serialize)]
pub struct Balance {
    pub currency: String,
    pub balance: String,
    pub available: String,
    pub hold: String,
}

#[derive(Deserialize, Debug, Clone, CandidType, Serialize)]
pub struct BidAsk {
    pub price: f64,
    pub size: f64,
}

#[derive(Deserialize, Debug, Clone, CandidType, Serialize)]
pub struct OrderBook {
    pub sequence: u64,
    pub bids: Vec<BidAsk>,
    pub asks: Vec<BidAsk>,
}

#[derive(Deserialize, Debug, Clone, CandidType, Serialize)]
pub struct GlobalPendingOrder {
    pub instrument_type: String,
    pub instrument_id: String,
    pub order_id: String,
    pub price: f64,
    pub size: f64,
    pub side: String,
    pub order_type: String,
    pub trade_mode: String,
    pub accumulated_fill_quantity: f64,
}
