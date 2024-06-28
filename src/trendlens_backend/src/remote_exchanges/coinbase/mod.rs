use crate::{
    chain_data::ChainData,
    exchange::{Candle, Exchange, ExchangeInfo},
    request_store::request::Response,
};

use super::{okx::api::GetInstrumentsRequest, ExchangeErrors, OpenData, UserData};

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

#[async_trait::async_trait]
impl UserData for Coinbase {
    async fn get_instruments(
        &self,
        req: GetInstrumentsRequest,
    ) -> Result<Response, ExchangeErrors> {
        Ok(Response::Instruments(vec![]))
    }
}

impl ChainData for Coinbase {
    fn key(&self) -> Exchange {
        Exchange::Coinbase
    }
}

impl ExchangeInfo for Coinbase {
    fn get_pairs(&self) -> Vec<crate::pair::Pair> {
        vec![]
    }
}
