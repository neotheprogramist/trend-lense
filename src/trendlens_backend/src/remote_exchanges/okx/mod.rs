use super::{ExchangeErrors, ExternalProvider};
use crate::exchange::Candle;
use crate::{api_client::ApiClient, Pair};
use api::IndexCandleStickRequest;
pub mod api;
pub mod response;

#[derive(Default)]
pub struct Okx {
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

    /// gets index name from global pair enum
    fn index_name(pair: Pair) -> Option<String> {
        match pair {
            Pair::BtcUsd => Some("BTC-USD".to_string()),
            Pair::EthUsd => Some("ETH-USD".to_string()),
        }
    }
}

impl ExternalProvider for Okx {
    async fn get_candles(
        &self,
        pair: Pair,
        _from: u64,
        _to: u64,
        interval: u32,
    ) -> Result<Vec<Candle>, ExchangeErrors> {
        let index_name = Okx::index_name(pair).ok_or_else(|| ExchangeErrors::MissingIndex)?;

        let candle_request = IndexCandleStickRequest {
            after_timestamp: None,
            before_timestamp: None,
            bar_size: Some(Okx::interval_string(interval)),
            index_name,
            results_limit: None,
        };

        let candle_response = self.api_client.call(candle_request).await?;

        Ok(candle_response
            .into_iter()
            .map(|concrete_candle| concrete_candle.into())
            .collect())
    }
}
