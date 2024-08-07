use std::{fmt::Display, str::FromStr};

use crate::{
    exchange::Candle,
    pair::Pair,
    remote_exchanges::{
        okx::api::InstrumentType,
        request::OrderSide,
        response::{
            ApiResponseWrapper, Balance, BidAsk as GlobalBidAsk, Instrument,
            OrderBook as GlobalOrderBook, OrderData,
        },
        ExchangeErrors,
    },
};
use candid::CandidType;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

impl FromStr for InstrumentStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "online" => Ok(InstrumentStatus::Online),
            "offline" => Ok(InstrumentStatus::Offline),
            "internal" => Ok(InstrumentStatus::Internal),
            "delisted" => Ok(InstrumentStatus::Delisted),
            _ => Err(format!("Unknown instrument status: {}", s)),
        }
    }
}

#[derive(Deserialize, Debug, Clone, CandidType)]
pub enum InstrumentStatus {
    Online,
    Offline,
    Internal,
    Delisted,
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub struct ConcreteInstrument {
    pub display_name: String
}

impl Into<Instrument> for ConcreteInstrument {
    fn into(self) -> Instrument {
        Instrument {
            instrument_id: Pair::from_str(self.display_name.as_str()).expect("Invalid pair"),
            instrument_type: InstrumentType::Spot,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct CoinbaseResponse<T>(pub T);

impl<R: DeserializeOwned> ApiResponseWrapper<R> for CoinbaseResponse<R> {
    fn extract_response(self) -> Result<R, ExchangeErrors> {
        Ok(self.0)
    }
}

#[derive(Deserialize, Serialize)]
pub struct OrderResponse {
    pub id: String,
    pub price: Option<String>,
    pub size: Option<String>,
    pub product_id: String,
    pub profile_id: Option<String>,
    pub side: String,
    pub funds: Option<String>,
    pub specified_funds: Option<String>,
    #[serde(rename = "type")]
    pub order_type: String,
    pub time_in_force: Option<String>,
    pub expire_time: Option<String>,
    pub post_only: bool,
    pub created_at: String,
    pub done_at: Option<String>,
    pub done_reason: Option<String>,
    pub reject_reason: Option<String>,
    pub fill_fees: String,
    pub filled_size: String,
    pub executed_value: Option<String>,
    pub status: String,
    pub settled: bool,
    pub stop: Option<String>,
    pub stop_price: Option<String>,
    pub funding_amount: Option<String>,
    pub client_oid: Option<String>,
    pub market_type: Option<String>,
    pub max_floor: Option<String>,
    pub secondary_order_id: Option<String>,
    pub stop_limit_price: Option<String>,
}

impl Into<OrderData> for OrderResponse {
    fn into(self) -> OrderData {
        OrderData {
            id: self.id,
            message: self.status,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Profile {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub active: bool,
    pub is_default: bool,
    pub created_at: String,
}

#[derive(Deserialize, Serialize)]
pub struct Account {
    pub id: String,
    pub currency: String,
    pub balance: String,
    pub hold: String,
    pub available: String,
    pub profile_id: String,
    pub trading_enabled: bool,
}

impl Into<Balance> for Account {
    fn into(self) -> Balance {
        Balance {
            currency: self.currency,
            balance: self.balance,
            available: self.available,
            hold: self.hold,
        }
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct BidAsk {
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub size: f64,
    pub orders_count: u32,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct OrderBook {
    pub asks: Vec<BidAsk>,
    pub bids: Vec<BidAsk>,
    pub sequence: u64,
    pub time: String,
}

impl Into<GlobalBidAsk> for BidAsk {
    fn into(self) -> GlobalBidAsk {
        GlobalBidAsk {
            price: self.price,
            size: self.size,
        }
    }
}

impl Into<GlobalOrderBook> for OrderBook {
    fn into(self) -> GlobalOrderBook {
        GlobalOrderBook {
            asks: self.asks.into_iter().map(|x| x.into()).collect(),
            bids: self.bids.into_iter().map(|x| x.into()).collect(),
            sequence: self.sequence,
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum OrderStatus {
    Open,
    Pending,
    Rejected,
    Done,
    Active,
    Received,
    All,
}

impl FromStr for OrderStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "open" => Ok(OrderStatus::Open),
            "pending" => Ok(OrderStatus::Pending),
            "rejected" => Ok(OrderStatus::Rejected),
            "done" => Ok(OrderStatus::Done),
            "active" => Ok(OrderStatus::Active),
            "received" => Ok(OrderStatus::Received),
            "all" => Ok(OrderStatus::All),
            _ => Err(format!("Unknown order status: {}", s)),
        }
    }
}

impl Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OrderStatus::Open => write!(f, "open"),
            OrderStatus::Pending => write!(f, "pending"),
            OrderStatus::Rejected => write!(f, "rejected"),
            OrderStatus::Done => write!(f, "done"),
            OrderStatus::Active => write!(f, "active"),
            OrderStatus::Received => write!(f, "received"),
            OrderStatus::All => write!(f, "all"),
        }
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct Order {
    pub id: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub price: Option<f64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub size: Option<f64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub funds: Option<f64>,
    pub product_id: String,
    #[serde_as(as = "DisplayFromStr")]
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub order_type: Option<String>,
    pub time_in_force: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    pub filled_size: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub status: OrderStatus,
}

#[derive(Deserialize, Serialize)]
pub struct CoinbaseCandle {
    pub time: u64,
    pub low: f64,
    pub high: f64,
    pub open: f64,
    pub close: f64,
    pub volume: f64,
}

impl Into<Candle> for CoinbaseCandle {
    fn into(self) -> Candle {
        Candle {
            timestamp: self.time,
            lowest_price: self.low,
            highest_price: self.high,
            open_price: self.open,
            close_price: self.close,
            volume: self.volume,
        }
    }
}
