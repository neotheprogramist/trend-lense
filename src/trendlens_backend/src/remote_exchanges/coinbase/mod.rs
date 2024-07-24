use super::response::OrderBook as GlobalOrderBook;
use super::{ApiRequest, ExchangeErrors, OpenData};
use crate::exchange::TimeVolume;
use crate::{
    api_client::ApiClient,
    chain_data::ChainData,
    exchange::{Candle, Exchange, ExchangeId},
    pair::Pair,
};
pub use auth::CoinbaseAuth;
use ic_cdk::api::management_canister::http_request::HttpMethod;
pub use request::GetProfileAccountsRequest;
pub use request::OrdersRequest as CoinbaseOrdersRequest;
pub use request::PostOrderBody;
pub use request::Statuses;
use request::{GetAllPairsRequest, GetOrderbookRequest, GetProductCandles};
use response::{CoinbaseCandle, CoinbaseResponse, ConcreteInstrument, OrderBook};

mod auth;
mod request;
mod response;
mod user;

#[derive(Default)]
pub struct Coinbase {
    api_client: ApiClient,
    auth: Option<auth::CoinbaseAuth>,
}

impl Coinbase {
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

#[async_trait::async_trait]
impl OpenData for Coinbase {
    async fn fetch_candles(
        &self,
        pair: &Pair,
        range: std::ops::Range<u64>,
        interval: u32,
    ) -> Result<Vec<Candle>, super::ExchangeErrors> {
        let candle_request = GetProductCandles {
            product_id: pair.to_string(),
            granularity: Some(interval.to_string()),
            start: Some(range.start.to_string()),
            end: Some(range.end.to_string()),
        };

        let candle_response = self
            .api_client
            .call::<CoinbaseResponse<Vec<CoinbaseCandle>>, GetProductCandles, CoinbaseAuth>(
                candle_request,
                self.auth.as_ref(),
            )
            .await?;

        Ok(candle_response
            .into_iter()
            .map(|concrete_candle| concrete_candle.into())
            .collect())
    }

    async fn get_taker_volume(
        &self,
        pair: &Pair,
        range: std::ops::Range<u64>,
    ) -> Result<Vec<TimeVolume>, ExchangeErrors> {
        self.fetch_candles(pair, range, 300).await.map(|candles| {
            candles
                .into_iter()
                .map(|candle| TimeVolume {
                    timestamp: candle.timestamp,
                    volume: candle.volume,
                })
                .collect()
        })
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

    async fn get_orderbook(
        &self,
        pair: &Pair,
        _size: u32,
    ) -> Result<GlobalOrderBook, ExchangeErrors> {
        let orderbook_request = GetOrderbookRequest {
            product_id: pair.to_string(),
            level: 1,
        };

        let response = self
            .api_client
            .call::<CoinbaseResponse<OrderBook>, GetOrderbookRequest, CoinbaseAuth>(
                orderbook_request,
                self.auth.as_ref(),
            )
            .await?;

        Ok(response.into())
    }
}

impl ExchangeId for Coinbase {
    fn exchange_id(&self) -> Exchange {
        Exchange::Coinbase
    }
}

impl ChainData for Coinbase {
    fn key(&self) -> Exchange {
        Exchange::Coinbase
    }
}
