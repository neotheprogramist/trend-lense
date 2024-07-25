use super::request::{OrderSide, OrderType, TradeMode};
use super::{ApiRequest, ExchangeErrors};
use crate::api_client::ApiClient;
use crate::chain_data::ChainData;
use crate::exchange::{Candle, Exchange};
use crate::Pair;
use api::IndexCandleStickRequest;
use auth::OkxAuth;
use ic_cdk::api::management_canister::http_request::HttpMethod;
use response::{ApiResponse, IndexCandleStick};

pub mod api;
pub mod auth;
pub mod open;
pub mod response;
pub mod user;

#[derive(Default)]
pub struct Okx {
    auth: Option<OkxAuth>,
    api_client: ApiClient,
}

impl Okx {
    pub async fn fetch_index_candles(
        &self,
        pair: &Pair,
        range: std::ops::Range<u64>,
        interval: u32,
    ) -> Result<Vec<Candle>, ExchangeErrors> {
        let index_name = Okx::instrument_id(pair).ok_or_else(|| ExchangeErrors::MissingIndex)?;

        ic_cdk::println!("index_name: {:?}", index_name);
        let candle_request = IndexCandleStickRequest {
            after_timestamp: None,
            before_timestamp: Some(range.start * 1000),
            bar_size: Some(Okx::interval_string(interval)),
            index_name,
            results_limit: None,
        };

        let candle_response = self
            .api_client
            .call::<ApiResponse<Vec<IndexCandleStick>>, IndexCandleStickRequest, OkxAuth>(
                candle_request,
                self.auth.as_ref(),
            )
            .await?;

        Ok(candle_response
            .into_iter()
            .map(|concrete_candle| concrete_candle.into())
            .collect())
    }

    /// gets interval string from u32 in minutes
    fn interval_string(interval: u32) -> String {
        match interval {
            0..=60 => "1m",
            61..=180 => "3m",
            181..=u32::MAX => "5m",
        }
        .to_string()
    }

    pub fn side_string(side: OrderSide) -> String {
        match side {
            OrderSide::Buy => "buy",
            OrderSide::Sell => "sell",
        }
        .to_string()
    }

    pub fn order_type_string(order_type: OrderType) -> String {
        match order_type {
            OrderType::Limit => "limit",
            OrderType::Market => "market",
            OrderType::PostOnly => "post_only",
            OrderType::Fok => "fok",
            OrderType::Ioc => "ioc",
        }
        .to_string()
    }

    pub fn trade_mode_string(trade_mode: TradeMode) -> String {
        match trade_mode {
            TradeMode::Cross => "cross",
            TradeMode::Isolated => "isolated",
            TradeMode::Cash => "cash",
            TradeMode::SpotIsolated => "spot_isolated",
        }
        .to_string()
    }

    /// gets index name from global pair enum
    pub fn instrument_id(pair: &Pair) -> Option<String> {
        return Some(format!("{}-{}", pair.base, pair.quote).to_uppercase());
    }

    pub fn get_signature_data<R: ApiRequest>(&self, request: R) -> String {
        let (qs, body) = if R::BODY {
            ("".to_string(), request.to_body())
        } else {
            let qs = request.to_query_string();

            let qs = if qs == "" {
                "".to_string()
            } else {
                format!("?{}", qs)
            };

            (qs, "".to_string())
        };

        let method_str = if R::METHOD == HttpMethod::GET {
            "GET"
        } else {
            "POST"
        };

        let api_url = format!("/{}{}", R::URI, qs);

        format!("{}{}{}", method_str, api_url, body).to_string()
    }
}

impl ChainData for Okx {
    fn key(&self) -> Exchange {
        Exchange::Okx
    }
}

#[cfg(test)]
mod test_okx_helpers {
    use super::*;

    #[test]
    fn test_okx_interval_string() {
        assert_eq!(Okx::interval_string(0), "1m");
        assert_eq!(Okx::interval_string(1), "1m");
        assert_eq!(Okx::interval_string(2), "3m");
        assert_eq!(Okx::interval_string(3), "3m");
        assert_eq!(Okx::interval_string(4), "5m");
        assert_eq!(Okx::interval_string(5), "5m");
        assert_eq!(Okx::interval_string(50), "5m");
        assert_eq!(Okx::interval_string(u32::MAX), "5m");
    }
}
