use super::okx::api::InstrumentType;
use crate::pair::Pair;
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Deserialize, Serialize, CandidType, Copy)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl Display for OrderSide {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OrderSide::Buy => write!(f, "buy"),
            OrderSide::Sell => write!(f, "sell"),
        }
    }
}

impl FromStr for OrderSide {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "buy" => Ok(OrderSide::Buy),
            "sell" => Ok(OrderSide::Sell),
            _ => Err(format!("Unknown order side: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, CandidType, Copy)]
pub enum PositionSide {
    Short,
    Long,
}

// no conditional bounding here, but with
// adding another exchange this should be hierarchical
#[derive(Debug, Clone, Deserialize, Serialize, CandidType, Copy)]
pub enum TradeMode {
    // Margin trading
    Cross,
    Isolated,

    // Non-margin trading
    Cash,
    SpotIsolated,
}

impl Display for TradeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TradeMode::Cross => write!(f, "cross"),
            TradeMode::Isolated => write!(f, "isolated"),
            TradeMode::Cash => write!(f, "cash"),
            TradeMode::SpotIsolated => write!(f, "spot_isolated"),
        }
    }
}

impl FromStr for TradeMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cross" => Ok(TradeMode::Cross),
            "isolated" => Ok(TradeMode::Isolated),
            "cash" => Ok(TradeMode::Cash),
            "spot_isolated" => Ok(TradeMode::SpotIsolated),
            _ => Err(format!("Unknown trade mode: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, CandidType, Copy)]
pub enum OrderType {
    Market,
    Limit,
    PostOnly,
    Fok,
    Ioc,
    // there are more
}


impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OrderType::Market => write!(f, "market"),
            OrderType::Limit => write!(f, "limit"),
            OrderType::PostOnly => write!(f, "post_only"),
            OrderType::Fok => write!(f, "fok"),
            OrderType::Ioc => write!(f, "ioc"),
        }
    }
}

impl FromStr for OrderType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "market" => Ok(OrderType::Market),
            "limit" => Ok(OrderType::Limit),
            "post_only" => Ok(OrderType::PostOnly),
            "fok" => Ok(OrderType::Fok),
            "ioc" => Ok(OrderType::Ioc),
            _ => Err(format!("Unknown order type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, CandidType)]
pub struct GeneralPostOrderRequest {
    pub instrument_id: Pair,
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
    pub currency: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, CandidType)]
pub struct GeneralOrdersListRequest {
    pub instrument_type: InstrumentType,
    pub instrument_id: Pair,
    pub pending: bool,
}
