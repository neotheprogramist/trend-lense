use crate::api_client::ApiClient;
use crate::chain_data::ChainData;
use crate::exchange::Exchange;
use crate::Pair;
use auth::OkxAuth;

use super::request::{OrderSide, OrderType, TradeMode};
use super::ApiRequest;

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
    /// gets interval string from u32 in minutes
    fn interval_string(interval: u32) -> String {
        match interval {
            0..=1 => "1m",
            2..=3 => "3m",
            4..=u32::MAX => "5m",
        }
        .to_string()
    }

    fn side_string(side: OrderSide) -> String {
        match side {
            OrderSide::Buy => "buy",
            OrderSide::Sell => "sell",
        }
        .to_string()
    }

    fn order_type_string(order_type: OrderType) -> String {
        match order_type {
            OrderType::Limit => "limit",
            OrderType::Market => "market",
            OrderType::PostOnly => "post_only",
            OrderType::Fok => "fok",
            OrderType::Ioc => "ioc",
        }
        .to_string()
    }

    fn trade_mode_string(trade_mode: TradeMode) -> String {
        match trade_mode {
            TradeMode::Cross => "cross",
            TradeMode::Isolated => "isolated",
            TradeMode::Cash => "cash",
            TradeMode::SpotIsolated => "spot_isolated",
        }
        .to_string()
    }

    /// gets index name from global pair enum
    fn index_name(pair: Pair) -> Option<String> {
        match pair {
            Pair::BtcUsd => Some("BTC-USD".to_string()),
            Pair::EthUsd => Some("ETH-USD".to_string()),
        }
    }

    pub fn get_signature_data<R: ApiRequest>(&self, request: R) -> String {
        let api_url = format!("/{}{}", R::URI, format!("?{}", request.to_query_string()));

        format!("{:?}{}{}", R::METHOD, R::BODY, api_url).to_string()
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
    use crate::Pair;

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

    #[test]
    fn test_okx_index_name() {
        assert_eq!(Okx::index_name(Pair::BtcUsd), Some("BTC-USD".to_string()));
        assert_eq!(Okx::index_name(Pair::EthUsd), Some("ETH-USD".to_string()));
    }
}
