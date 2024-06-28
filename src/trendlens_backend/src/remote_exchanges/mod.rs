use crate::chain_data::ChainData;
use crate::exchange::Candle;
use crate::request_store::request::Response;
use crate::{api_client::ApiClientErrors, Pair};
use candid::CandidType;
use ic_cdk::api::management_canister::http_request::{HttpHeader, HttpMethod};
use okx::api::GetInstrumentsRequest;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod coinbase;
pub mod okx;

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

    // // consider
    // async fn get_account_instruments(&self) -> Result<Vec<Instrument>, ExchangeErrors>;
}

#[async_trait::async_trait]
pub trait AuthorizedData {
    async fn get_instruments(
        &self,
        request: GetInstrumentsRequest,
    ) -> Result<Response, ExchangeErrors>;
}

pub trait ApiRequest: Serialize {
    const METHOD: HttpMethod;
    const URI: &'static str;
    const HOST: &'static str;
    const PUBLIC: bool;

    type Response: for<'de> Deserialize<'de>;

    fn to_query_string(&self) -> String
    where
        Self: Serialize + Sized,
    {
        serde_qs::to_string(self).unwrap()
    }
}

pub trait Authorizable {
    fn get_auth_headers(&self) -> Vec<HttpHeader>;
}

pub trait UpdateExchange: OpenData + ChainData + AuthorizedData {}
impl<T> UpdateExchange for T where T: OpenData + ChainData + AuthorizedData {}
