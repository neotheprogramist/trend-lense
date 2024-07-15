use auth::CoinbaseAuth;
use request::GetAllPairsRequest;
use response::CoinbaseResponse;

use crate::{
    api_client::ApiClient,
    chain_data::ChainData,
    exchange::{Candle, Exchange, ExchangeId},
    request_store::request::Response,
};

use super::{request::GeneralInstrumentsRequest, ExchangeErrors, OpenData, UserData};
use response::ConcreteInstrument;

mod auth;
mod request;
mod response;

#[derive(Default)]
pub struct Coinbase {
    api_client: ApiClient,
    auth: Option<auth::CoinbaseAuth>,
}

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

    async fn get_public_instruments(
        &self,
        _request: crate::remote_exchanges::request::GeneralInstrumentsRequest,
    ) -> Result<Vec<crate::remote_exchanges::response::Instrument>, super::ExchangeErrors> {
        let coinbase_instruction = GetAllPairsRequest {};

        let response = self
            .api_client
            .call::<CoinbaseResponse<Vec<ConcreteInstrument>>, GetAllPairsRequest, CoinbaseAuth>(
                coinbase_instruction,
                self.auth.as_ref(),
            )
            .await?;

        Ok(response.into_iter().map(Into::into).collect())
    }
}

impl ExchangeId for Coinbase {
    fn exchange_id(&self) -> Exchange {
        Exchange::Coinbase
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
