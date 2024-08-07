use serde_this_or_that::as_f64;
use serde_this_or_that::as_u64;
use std::{fmt::Display, str::FromStr};

use crate::{
    exchange::Candle,
    pair::Pair,
    remote_exchanges::{
        request::{OrderSide, OrderType, TradeMode},
        response::{
            ApiResponseWrapper, Balance, BidAsk as GlobalBidAsk, Instrument,
            OrderBook as GlobalOrderBook,
        },
        ExchangeErrors,
    },
};
use candid::CandidType;
use serde::{self, de::DeserializeOwned, Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use super::api::InstrumentType;

#[serde_as]
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct ApiResponse<T> {
    #[serde(deserialize_with = "as_u64")]
    pub code: u64,
    pub msg: String,
    pub data: T,
}

impl<R: DeserializeOwned> ApiResponseWrapper<R> for ApiResponse<R> {
    fn extract_response(self) -> Result<R, ExchangeErrors> {
        Ok(self.data)
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct BidAsk {
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub size: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub deprecated: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub orders_count: u32,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct OrderBook {
    pub asks: Vec<BidAsk>,
    pub bids: Vec<BidAsk>,
    #[serde(rename = "ts")]
    #[serde_as(as = "DisplayFromStr")]
    pub timestamp: u64,
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
            sequence: self.timestamp,
        }
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct IndexCandleStick {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "ts")]
    pub timestamp: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "o")]
    pub open_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "h")]
    pub highest_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "l")]
    pub lowest_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "c")]
    pub close_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub confirm: u8,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CandleStick {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "ts")]
    pub timestamp: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "o")]
    pub open_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "h")]
    pub highest_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "l")]
    pub lowest_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "c")]
    pub close_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "vol")]
    pub volume: f64,
    #[serde(rename = "volCcy")]
    pub volume_currency: String,
    #[serde(rename = "volCcyQuote")]
    pub volume_currency_quote: String,
    pub confirm: String,
}

impl Into<Candle> for CandleStick {
    fn into(self) -> Candle {
        Candle {
            close_price: self.close_price,
            highest_price: self.highest_price,
            lowest_price: self.lowest_price,
            open_price: self.open_price,
            timestamp: self.timestamp / 1000,
            volume: self.volume,
        }
    }
}

impl Into<Candle> for IndexCandleStick {
    fn into(self) -> Candle {
        Candle {
            close_price: self.close_price,
            highest_price: self.highest_price,
            lowest_price: self.lowest_price,
            open_price: self.open_price,
            timestamp: self.timestamp / 1000,
            volume: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct PlaceOrderDetails {
    #[serde(rename = "ordId")]
    pub order_id: String,
    #[serde(rename = "clOrdId")]
    pub client_order_id: String,
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "sCode")]
    pub status_code: String,
    #[serde(rename = "sMsg")]
    pub status_message: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct Order {
    #[serde(rename = "instType")]
    #[serde_as(as = "DisplayFromStr")]
    pub instrument_type: InstrumentType,
    #[serde(rename = "instId")]
    pub instrument_id: String,
    #[serde(rename = "tgtCcy")]
    pub order_quantity_currency: String,
    #[serde(rename = "ordId")]
    pub order_id: String,
    #[serde(rename = "px")]
    #[serde(deserialize_with = "as_f64")]
    pub price: f64,
    #[serde(rename = "sz")]
    #[serde_as(as = "DisplayFromStr")]
    pub size: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub side: OrderSide,
    #[serde(rename = "ordType")]
    #[serde_as(as = "DisplayFromStr")]
    pub order_type: OrderType,
    #[serde(rename = "tdMode")]
    #[serde_as(as = "DisplayFromStr")]
    pub trade_mode: TradeMode,
    #[serde(rename = "accFillSz")]
    #[serde_as(as = "DisplayFromStr")]
    pub accumulated_fill_quantity: f64,
    #[serde(rename = "avgPx")]
    #[serde(deserialize_with = "as_f64")]
    pub average_filled_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub state: OrderState,
}

#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub enum OrderState {
    Pending,
    Done,
    Canceled,
    Filled,
    MmpCanceled,
    Live,
    PartiallyFilled,
}

impl FromStr for OrderState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(OrderState::Pending),
            "done" => Ok(OrderState::Done),
            "canceled" => Ok(OrderState::Canceled),
            "filled" => Ok(OrderState::Filled),
            "mmp_canceled" => Ok(OrderState::MmpCanceled),
            "live" => Ok(OrderState::Live),
            "partially_filled" => Ok(OrderState::PartiallyFilled),
            _ => Err("Unknown order state".to_string()),
        }
    }
}

impl Display for OrderState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            OrderState::Pending => "pending",
            OrderState::Done => "done",
            OrderState::Canceled => "canceled",
            OrderState::Filled => "filled",
            OrderState::MmpCanceled => "mmp_canceled",
            OrderState::Live => "live",
            OrderState::PartiallyFilled => "partially_filled",
        };

        write!(f, "{}", state)
    }
}

#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct AccountInfo {
    #[serde(rename = "uTime")]
    update_time: String,
    #[serde(rename = "totalEq")]
    total_equity: String,
    #[serde(rename = "isoEq")]
    isolated_margin_equity: Option<String>,
    #[serde(rename = "adjEq")]
    adjusted_equity: Option<String>,
    #[serde(rename = "ordFroz")]
    cross_margin_frozen: Option<String>,
    #[serde(rename = "imr")]
    initial_margin_requirement: Option<String>,
    #[serde(rename = "mmr")]
    maintenance_margin_requirement: Option<String>,
    #[serde(rename = "borrowFroz")]
    potential_borrowing_imr: Option<String>,
    #[serde(rename = "mgnRatio")]
    margin_ratio: Option<String>,
    #[serde(rename = "notionalUsd")]
    notional_value_usd: Option<String>,
    #[serde(rename = "upl")]
    unrealized_profit_loss: Option<String>,
    details: Vec<AssetDetail>,
}

impl Into<Vec<Balance>> for AccountInfo {
    fn into(self) -> Vec<Balance> {
        self.details
            .into_iter()
            .map(|d| Balance {
                currency: d.currency,
                available: d.equity,
                balance: d.cash_balance,
                hold: "0".to_string(),
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct AssetDetail {
    #[serde(rename = "ccy")]
    currency: String,
    #[serde(rename = "eq")]
    equity: String,
    #[serde(rename = "cashBal")]
    cash_balance: String,
    #[serde(rename = "uTime")]
    update_time: String,
}

impl FromStr for Pair {
    type Err = ExchangeErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let all_alphanumeric = s.chars().all(char::is_alphanumeric);

        // handle it later
        if all_alphanumeric {
            return Err(ExchangeErrors::UnsupportedPairFormat);
        }

        let splitted = s
            .split(|pat: char| !pat.is_alphanumeric())
            .collect::<Vec<_>>();

        if splitted.len() != 2 {
            return Err(ExchangeErrors::UnsupportedPairFormat);
        }

        Ok(Pair {
            base: splitted[0].to_string(),
            quote: splitted[1].to_string(),
        })
    }
}

#[cfg(test)]
mod pair_from_str_test {
    use super::*;

    #[test]
    fn test_conversion_success() {
        assert_eq!(
            Pair::from_str("btc_usd").unwrap(),
            Pair {
                base: "BTC".to_string(),
                quote: "USD".to_string()
            }
        );
        assert_eq!(
            Pair::from_str("btc.usd").unwrap(),
            Pair {
                base: "BTC".to_string(),
                quote: "USD".to_string()
            }
        );
        assert_eq!(
            Pair::from_str("btc-usdt").unwrap(),
            Pair {
                base: "BTC".to_string(),
                quote: "USDT".to_string()
            }
        );
        assert_eq!(
            Pair::from_str("btc.usdt").unwrap(),
            Pair {
                base: "BTC".to_string(),
                quote: "USDT".to_string()
            }
        );
    }
}

#[serde_as]
#[derive(Deserialize, Debug, Clone, CandidType)]
pub struct ConcreteInstrument {
    #[serde(rename = "instId")]
    #[serde_as(as = "DisplayFromStr")]
    pub instrument_id: Pair,
    #[serde(rename = "instType")]
    #[serde_as(as = "DisplayFromStr")]
    pub instrument_type: InstrumentType,
    #[serde(rename = "uly")]
    pub underlying: String,
    #[serde(rename = "instFamily")]
    pub instrument_family: String,
    #[serde(rename = "baseCcy")]
    base_currency: Option<String>,
    #[serde(rename = "quoteCcy")]
    quote_currency: Option<String>
}

impl Into<Instrument> for ConcreteInstrument {
    fn into(self) -> Instrument {
        Instrument {
            instrument_id: self.instrument_id,
            instrument_type: self.instrument_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let response = r#"{"code":"0","msg":"success","data":[{"ts":"1620000000000","o":"1.0","h":"1.0","l":"1.0","c":"1.0","confirm":"1"}]}"#;
        let response: ApiResponse<Vec<IndexCandleStick>> = serde_json::from_str(response).unwrap();

        assert_eq!(response.code, 0);
        assert_eq!(response.msg, "success");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].timestamp, 1620000000000);
        assert_eq!(response.data[0].open_price, 1.0);
        assert_eq!(response.data[0].highest_price, 1.0);
        assert_eq!(response.data[0].lowest_price, 1.0);
        assert_eq!(response.data[0].close_price, 1.0);
        assert_eq!(response.data[0].confirm, 1);
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct TakerVolume {
    #[serde(rename = "ts")]
    #[serde_as(as = "DisplayFromStr")]
    pub timestamp: u64,
    #[serde(rename = "buyVol")]
    #[serde_as(as = "DisplayFromStr")]
    pub buy_volume: f64,
    #[serde(rename = "sellVol")]
    #[serde_as(as = "DisplayFromStr")]
    pub sell_volume: f64,
}
