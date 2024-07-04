use crate::{
    chain_data::ChainData,
    exchange::{Candle, Exchange, ExchangeInfo},
    request_store::request::Response,
};

use super::{request::GeneralInstrumentsRequest, ExchangeErrors, OpenData, UserData};

#[derive(Default)]
pub struct Coinbase;

#[async_trait::async_trait]
impl OpenData for Coinbase {
    async fn fetch_candles(
        &self,
        _pair: crate::pair::Pair,
        _range: std::ops::Range<u64>,
        _interval: u32,
    ) -> Result<Vec<Candle>, super::ExchangeErrors> {
        Ok(vec![])
    }
}

#[async_trait::async_trait]
impl UserData for Coinbase {
    async fn get_instruments(
        &self,
        _req: GeneralInstrumentsRequest,
    ) -> Result<Response, ExchangeErrors> {
        Ok(Response::Instruments(vec![]))
    }

    async fn get_balance(
        &self,
        _request: crate::remote_exchanges::request::GeneralBalanceRequest,
    ) -> Result<Response, ExchangeErrors> {
        todo!()
    }

    async fn post_order(
        &self,
        _request: crate::remote_exchanges::request::GeneralPostOrderRequest,
    ) -> Result<Response, ExchangeErrors> {
        todo!()
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
