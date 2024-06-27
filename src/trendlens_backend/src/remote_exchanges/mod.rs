use crate::chain_data::ChainData;
use crate::exchange::{Candle, Exchange};
use crate::{api_client::ApiClientErrors, Pair};
use candid::Principal;
use ic_cdk::api::management_canister::http_request::HttpMethod;
use okx::api::Instrument;
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

// provider is not descriptive enough, maybe change name or split traits
#[async_trait::async_trait]
pub trait ExternalProvider {
    async fn fetch_candles(
        &self,
        pair: Pair,
        range: std::ops::Range<u64>,
        interval: u32,
    ) -> Result<Vec<Candle>, ExchangeErrors>;

    // // consider
    // async fn get_account_instruments(&self) -> Result<Vec<Instrument>, ExchangeErrors>;
}

pub trait RequestStore {
    fn store_request(&self, exchange: Exchange, identity: &Principal) -> Option<u8>;
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
