use std::{fmt, str::FromStr};

use super::response::CandleStick;
use crate::remote_exchanges::ApiRequest;
use candid::CandidType;
use ic_cdk::api::management_canister::http_request::HttpMethod;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};

#[derive(Debug, Clone, CandidType)]
pub enum InstrumentType {
    Spot,
    Futures,
    Swap,
    Option,
    Margin,
}

impl FromStr for InstrumentType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SPOT" => Ok(InstrumentType::Spot),
            "FUTURES" => Ok(InstrumentType::Futures),
            "SWAP" => Ok(InstrumentType::Swap),
            "OPTION" => Ok(InstrumentType::Option),
            "MARGIN" => Ok(InstrumentType::Margin),
            _ => Err(format!("Unknown instrument type: {}", s)),
        }
    }
}

impl fmt::Display for InstrumentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstrumentType::Spot => write!(f, "SPOT"),
            InstrumentType::Margin => write!(f, "MARGIN"),
            InstrumentType::Futures => write!(f, "FUTURES"),
            InstrumentType::Option => write!(f, "OPTION"),
            InstrumentType::Swap => write!(f, "SWAP"),
        }
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct GetInstrumentsRequest {
    #[serde(rename = "instType")]
    #[serde_as(as = "DisplayFromStr")]
    pub instrument_type: InstrumentType,
    #[serde(rename = "instId")]
    pub instrument_id: Option<String>,
    // for now skipped conditional fields
}

#[serde_as]
#[derive(Deserialize, Debug, Clone, CandidType)]
pub struct Instrument {
    #[serde(rename = "instId")]
    pub instrument_id: String,
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
    quote_currency: Option<String>,
    #[serde(rename = "settleCcy")]
    settlement_currency: Option<String>,
    #[serde(rename = "ctVal")]
    contract_value: Option<String>,
    #[serde(rename = "ctMult")]
    contract_multiplier: Option<String>,
    #[serde(rename = "ctValCcy")]
    contract_value_currency: Option<String>,
    #[serde(rename = "optType")]
    option_type: Option<String>,
    #[serde(rename = "stk")]
    strike_price: Option<String>,
    #[serde(rename = "listTime")]
    listing_time: Option<String>,
    #[serde(rename = "expTime")]
    expiry_time: Option<String>,
    #[serde(rename = "lever")]
    leverage: Option<String>,
    #[serde(rename = "tickSz")]
    tick_size: String,
    #[serde(rename = "lotSz")]
    lot_size: String,
    #[serde(rename = "minSz")]
    minimum_order_size: String,
    #[serde(rename = "ctType")]
    contract_type: Option<String>,
    #[serde(rename = "state")]
    state: String,
    #[serde(rename = "maxLmtSz")]
    max_limit_size: String,
    #[serde(rename = "maxMktSz")]
    max_market_size: String,
    #[serde(rename = "maxLmtAmt")]
    max_limit_amount: Option<String>,
    #[serde(rename = "maxMktAmt")]
    max_market_amount: Option<String>,
    #[serde(rename = "maxTwapSz")]
    max_twap_size: String,
    #[serde(rename = "maxIcebergSz")]
    max_iceberg_size: String,
    #[serde(rename = "maxTriggerSz")]
    max_trigger_size: String,
    #[serde(rename = "maxStopSz")]
    max_stop_size: String,
}

impl ApiRequest for GetInstrumentsRequest {
    const METHOD: HttpMethod = HttpMethod::GET;
    const URI: &'static str = "api/v5/account/instruments";
    const HOST: &'static str = "www.okx.com";
    const PUBLIC: bool = false;

    type Response = Vec<Instrument>;
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexCandleStickRequest {
    #[serde(rename = "instId")]
    pub index_name: String,
    #[serde(rename = "after")]
    pub after_timestamp: Option<u64>,
    #[serde(rename = "before")]
    pub before_timestamp: Option<u64>,
    #[serde(rename = "bar")]
    pub bar_size: Option<String>,
    #[serde(rename = "limit")]
    pub results_limit: Option<u8>,
}

impl ApiRequest for IndexCandleStickRequest {
    const METHOD: HttpMethod = HttpMethod::GET;
    const URI: &'static str = "api/v5/market/index-candles";
    const HOST: &'static str = "www.okx.com";
    const PUBLIC: bool = true;

    type Response = Vec<CandleStick>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_request() {
        assert_eq!(IndexCandleStickRequest::HOST, "www.okx.com");

        assert_eq!(IndexCandleStickRequest::URI, "api/v5/market/index-candles");

        assert_eq!(IndexCandleStickRequest::METHOD, HttpMethod::GET);
    }

    #[test]
    fn test_query_string_full() {
        let request = IndexCandleStickRequest {
            index_name: "BTC-USD".to_string(),
            after_timestamp: Some(1),
            before_timestamp: Some(2),
            bar_size: Some("1m".to_string()),
            results_limit: Some(1),
        };

        let query_string = request.to_query_string();

        assert_eq!(
            query_string,
            "instId=BTC-USD&after=1&before=2&bar=1m&limit=1"
        );
    }

    #[test]
    fn test_query_string_missing() {
        let request = IndexCandleStickRequest {
            index_name: "BTC-USD".to_string(),
            after_timestamp: None,
            before_timestamp: None,
            bar_size: None,
            results_limit: None,
        };

        let query_string = request.to_query_string();

        assert_eq!(query_string, "instId=BTC-USD");
    }
}
