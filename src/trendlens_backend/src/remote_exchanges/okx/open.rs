use crate::{exchange::Candle, pair::Pair, remote_exchanges::{ExchangeErrors, OpenData}};

use super::{api::IndexCandleStickRequest, Okx};


#[async_trait::async_trait]
impl OpenData for Okx {
    async fn fetch_candles(
        &self,
        pair: Pair,
        range: std::ops::Range<u64>,
        interval: u32,
    ) -> Result<Vec<Candle>, ExchangeErrors> {
        let index_name = Okx::index_name(pair).ok_or_else(|| ExchangeErrors::MissingIndex)?;

        let candle_request = IndexCandleStickRequest {
            after_timestamp: None,
            before_timestamp: Some(range.start * 1000),
            bar_size: Some(Okx::interval_string(interval)),
            index_name,
            results_limit: None,
        };

        let candle_response = self
            .api_client
            .call(candle_request, self.auth.as_ref())
            .await?;

        Ok(candle_response
            .into_iter()
            .map(|concrete_candle| concrete_candle.into())
            .collect())
    }
}
