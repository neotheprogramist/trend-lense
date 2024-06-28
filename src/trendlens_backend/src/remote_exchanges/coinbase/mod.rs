use crate::{api_client::ApiClient, chain_data::ChainData, exchange::Exchange};

use super::ExternalProvider;

#[derive(Default)]
pub struct Coinbase {
    api_client: ApiClient,
}

#[async_trait::async_trait]
impl ExternalProvider for Coinbase {
    async fn fetch_candles(
        &self,
        pair: crate::pair::Pair,
        range: std::ops::Range<u64>,
        interval: u32,
    ) -> Result<Vec<crate::exchange::Candle>, super::ExchangeErrors> {
        Ok(vec![])
    }
}

impl ChainData for Coinbase {
    fn key(&self) -> Exchange {
        Exchange::Coinbase
    }
}
