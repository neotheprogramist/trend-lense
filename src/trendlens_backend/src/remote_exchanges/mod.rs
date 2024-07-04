use crate::chain_data::ChainData;
use crate::exchange::{Candle, ExchangeInfo};
use crate::request_store::request::Response;
use crate::{api_client::ApiClientErrors, Pair};
use candid::CandidType;
use ic_cdk::api::management_canister::http_request::{HttpHeader, HttpMethod};
use request::{GeneralBalanceRequest, GeneralInstrumentsRequest};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod coinbase;
pub mod okx;
pub mod request;

#[derive(Debug, Error, CandidType)]
pub enum ExchangeErrors {
    #[error("api error")]
    ApiClientError(#[from] ApiClientErrors),
    #[error("this exchange has no such index")]
    MissingIndex,
}

#[async_trait::async_trait]
pub trait OpenData {
    async fn fetch_candles(
        &self,
        pair: Pair,
        range: std::ops::Range<u64>,
        interval: u32,
    ) -> Result<Vec<Candle>, ExchangeErrors>;
}

#[async_trait::async_trait]
pub trait UserData {
    async fn get_instruments(
        &self,
        request: GeneralInstrumentsRequest,
    ) -> Result<Response, ExchangeErrors>;

    async fn get_balance(&self, request: GeneralBalanceRequest)
        -> Result<Response, ExchangeErrors>;
}

pub trait ApiRequest: Serialize {
    const METHOD: HttpMethod;
    const URI: &'static str;
    const HOST: &'static str;
    const BODY: &'static str = "";
    const PUBLIC: bool;

    type Response: for<'de> Deserialize<'de>;

    fn to_query_string(&self) -> String
    where
        Self: Serialize + Sized,
    {
        serde_qs::to_string(self).unwrap()
    }
}

pub trait Authorize {
    fn get_auth_headers(&self) -> Vec<HttpHeader>;
}

pub trait UpdateExchange: OpenData + UserData + ChainData + ExchangeInfo {}
impl<T> UpdateExchange for T where T: OpenData + UserData + ChainData + ExchangeInfo {}
