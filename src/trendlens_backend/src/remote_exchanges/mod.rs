use crate::exchange::Candle;
use crate::remote_exchanges::response::Instrument;
use crate::request_store::request::Response;
use crate::{api_client::ApiClientErrors, Pair};
use candid::CandidType;
use ic_cdk::api::management_canister::http_request::{HttpHeader, HttpMethod};
use okx::response::OrderBook;
use request::{GeneralBalanceRequest, GeneralInstrumentsRequest, GeneralPostOrderRequest};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod coinbase;
pub mod okx;
pub mod request;
pub mod response;

#[derive(Debug, Error, CandidType)]
pub enum ExchangeErrors {
    #[error("api error")]
    ApiClientError(#[from] ApiClientErrors),
    #[error("this exchange has no such index")]
    MissingIndex,
    #[error("this exchange has no such pair")]
    MissingPair,
    #[error("provided timestamps are invalid")]
    InvalidTimestamps,
    #[error("pair format given is not supported")]
    UnsupportedPairFormat,
    #[error("could not deserialize: {message}")]
    DeserializationFailed {
        message: String,
    },
    #[error("api key not found")]
    MissingApiKey,
}

#[async_trait::async_trait]
pub trait OpenData {
    async fn get_public_instruments(
        &self,
        request: GeneralInstrumentsRequest,
    ) -> Result<Vec<Instrument>, ExchangeErrors>;

    async fn fetch_candles(
        &self,
        pair: &Pair,
        range: std::ops::Range<u64>,
        interval: u32,
    ) -> Result<Vec<Candle>, ExchangeErrors>;

    async fn get_orderbook(&self, pair: &Pair, size: u32) -> Result<Vec<OrderBook>, ExchangeErrors>;
}

#[async_trait::async_trait]
pub trait UserData {
    async fn get_instruments(
        &self,
        request: GeneralInstrumentsRequest,
    ) -> Result<Response, ExchangeErrors>;

    async fn get_balance(&self, request: GeneralBalanceRequest)
        -> Result<Response, ExchangeErrors>;

    async fn post_order(
        &self,
        request: GeneralPostOrderRequest,
    ) -> Result<Response, ExchangeErrors>;
}

pub trait ApiRequest: Serialize {
    const METHOD: HttpMethod;
    const URI: &'static str;
    const HOST: &'static str;
    const BODY: bool;
    const PUBLIC: bool;

    type Response: for<'de> Deserialize<'de>;

    fn to_query_string(&self) -> String
    where
        Self: Serialize + Sized,
    {
        serde_qs::to_string(self).unwrap()
    }

    fn to_body(&self) -> String
    where
        Self: Serialize + Sized,
    {
        serde_json::to_string(self).unwrap()
    }
}

pub trait Authorize {
    fn get_auth_headers(&self) -> Vec<HttpHeader>;
}
