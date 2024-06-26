use crate::chain_data::ChainData;
use crate::exchange::Candle;
use crate::{api_client::ApiClientErrors, Pair};
use ic_cdk::api::management_canister::http_request::HttpMethod;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod coinbase;
pub mod okx;

#[derive(Debug, Error)]
pub enum ExchangeErrors {
    #[error("api error")]
    ApiClientError(#[from] ApiClientErrors),
    #[error("this exchange has no such index")]
    MissingIndex,
}

#[async_trait::async_trait]
pub trait ExternalProvider {
    async fn fetch_candles(
        &self,
        pair: Pair,
        range: std::ops::Range<u64>,
        interval: u32,
    ) -> Result<Vec<Candle>, ExchangeErrors>;
}

pub trait ApiRequest: Serialize {
    const METHOD: HttpMethod;
    const URI: &'static str;
    const HOST: &'static str;

    type Response: for<'de> Deserialize<'de>;

    fn to_query_string(&self) -> String
    where
        Self: Serialize + Sized,
    {
        serde_qs::to_string(self).unwrap()
    }
}

pub trait UpdateExchange: ExternalProvider + ChainData {}
impl<T> UpdateExchange for T where T: ExternalProvider + ChainData {}
