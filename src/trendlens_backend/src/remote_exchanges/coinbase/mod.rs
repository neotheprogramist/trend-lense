use crate::{api_client::ApiClient, chain_data::ChainData, exchange::{Candle, Exchange}};

use super::{ApiRequest, OpenData};

#[derive(Default)]
pub struct Coinbase;

#[async_trait::async_trait]
impl OpenData for Coinbase {
    async fn fetch_candles(
        &self,
        pair: crate::pair::Pair,
        range: std::ops::Range<u64>,
        interval: u32,
    ) -> Result<Vec<Candle>, super::ExchangeErrors> {
        Ok(vec![])
    }
}

impl ChainData for Coinbase {
    fn key(&self) -> Exchange {
        Exchange::Coinbase
    }
}
