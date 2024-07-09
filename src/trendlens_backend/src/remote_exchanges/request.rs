use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::pair::Pair;

use super::okx::api::InstrumentType;

#[derive(Debug, Clone, Deserialize, Serialize, CandidType)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Deserialize, Serialize, CandidType)]
pub enum PositionSide {
    Short,
    Long,
}

// no conditional bounding here, but with
// adding another exchange this should be hierarchical
#[derive(Debug, Clone, Deserialize, Serialize, CandidType)]
pub enum TradeMode {
    // Margin trading
    Cross,
    Isolated,

    // Non-margin trading
    Cash,
    SpotIsolated,
}

#[derive(Debug, Clone, Deserialize, Serialize, CandidType)]
pub enum OrderType {
    Market,
    Limit,
    PostOnly,
    Fok,
    Ioc,
    // there are more
}

#[derive(Debug, Clone, Deserialize, Serialize, CandidType)]
pub struct GeneralPostOrderRequest {
    pub instrument_id: String,
    pub trade_mode: TradeMode,
    pub side: OrderSide,
    pub margin_currency: Option<String>,
    pub position_side: Option<PositionSide>,
    pub order_type: OrderType,
    pub size: f64,
    pub order_price: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, CandidType)]
pub struct GeneralInstrumentsRequest {
    pub instrument_type: InstrumentType,
    pub instrument_id: Option<Pair>,
}

#[derive(Debug, Clone, Deserialize, Serialize, CandidType)]
pub struct GeneralBalanceRequest {
    pub currency: Vec<String>,
}
