use crate::exchange::Candle;
use candid::CandidType;
use serde::{self, Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use super::api::InstrumentType;

#[serde_as]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ApiResponse<T> {
    #[serde_as(as = "DisplayFromStr")]
    pub code: u32,
    pub msg: String,
    pub data: T,
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
    pub confirm: u8,
}

impl Into<Candle> for CandleStick {
    fn into(self) -> Candle {
        Candle {
            close_price: self.close_price,
            highest_price: self.highest_price,
            lowest_price: self.lowest_price,
            open_price: self.open_price,
            timestamp: self.timestamp / 1000,
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

#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct PlaceOrderResponse {
    pub code: String,
    pub msg: String,
    pub data: Vec<PlaceOrderDetails>,
    pub in_time: String,
    pub out_time: String,
}

#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct AccountInfo {
    #[serde(rename = "uTime")]
    update_time: String,
    #[serde(rename = "totalEq")]
    total_equity: String,
    #[serde(rename = "isoEq")]
    isolated_margin_equity: String,
    #[serde(rename = "adjEq")]
    adjusted_equity: String,
    #[serde(rename = "ordFroz")]
    cross_margin_frozen: String,
    #[serde(rename = "imr")]
    initial_margin_requirement: String,
    #[serde(rename = "mmr")]
    maintenance_margin_requirement: String,
    #[serde(rename = "borrowFroz")]
    potential_borrowing_imr: String,
    #[serde(rename = "mgnRatio")]
    margin_ratio: String,
    #[serde(rename = "notionalUsd")]
    notional_value_usd: String,
    #[serde(rename = "upl")]
    unrealized_profit_loss: String,
    details: Vec<AssetDetail>,
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
    #[serde(rename = "isoEq")]
    isolated_margin_equity: String,
    #[serde(rename = "availEq")]
    available_equity: String,
    #[serde(rename = "disEq")]
    discount_equity: String,
    #[serde(rename = "fixedBal")]
    fixed_balance: String,
    #[serde(rename = "availBal")]
    available_balance: String,
    #[serde(rename = "frozenBal")]
    frozen_balance: String,
    #[serde(rename = "ordFrozen")]
    margin_frozen: String,
    #[serde(rename = "liab")]
    liabilities: String,
    #[serde(rename = "upl")]
    unrealized_profit_loss: String,
    #[serde(rename = "uplLiab")]
    upl_liabilities: String,
    #[serde(rename = "crossLiab")]
    cross_liabilities: String,
    #[serde(rename = "rewardBal")]
    reward_balance: String,
    #[serde(rename = "isoLiab")]
    isolated_liabilities: String,
    #[serde(rename = "mgnRatio")]
    margin_ratio: String,
    #[serde(rename = "interest")]
    accrued_interest: String,
    #[serde(rename = "twap")]
    twap: String,
    #[serde(rename = "maxLoan")]
    max_loan: String,
    #[serde(rename = "eqUsd")]
    equity_usd: String,
    #[serde(rename = "borrowFroz")]
    potential_borrowing_imr: String,
    #[serde(rename = "notionalLever")]
    leverage: String,
    #[serde(rename = "stgyEq")]
    strategy_equity: String,
    #[serde(rename = "isoUpl")]
    isolated_unrealized_profit_loss: String,
    #[serde(rename = "spotInUseAmt")]
    spot_in_use_amount: String,
    #[serde(rename = "clSpotInUseAmt")]
    user_defined_spot_risk_offset_amount: String,
    #[serde(rename = "maxSpotInUseAmt")]
    max_spot_risk_offset_amount: String,
    #[serde(rename = "spotIsoBal")]
    spot_isolated_balance: String,
    #[serde(rename = "imr")]
    initial_margin_requirement: String,
    #[serde(rename = "mmr")]
    maintenance_margin_requirement: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let response = r#"{"code":"0","msg":"success","data":[{"ts":"1620000000000","o":"1.0","h":"1.0","l":"1.0","c":"1.0","confirm":"1"}]}"#;
        let response: ApiResponse<Vec<CandleStick>> = serde_json::from_str(response).unwrap();

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
