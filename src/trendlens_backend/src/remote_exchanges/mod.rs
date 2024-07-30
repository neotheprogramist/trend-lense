use crate::exchange::{Candle, TimeVolume};
use crate::remote_exchanges::response::Instrument;
use crate::request_store::request::Response;
use crate::{api_client::ApiClientErrors, Pair};
use candid::CandidType;
use ic_cdk::api::management_canister::http_request::{HttpHeader, HttpMethod};
use request::{
    GeneralBalanceRequest, GeneralInstrumentsRequest, GeneralOrdersListRequest,
    GeneralPostOrderRequest,
};
use response::OrderBook;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

pub mod coinbase;
pub mod okx;
pub mod request;
pub mod response;

#[derive(Debug, Error, CandidType)]
pub enum ExchangeErrors {
    #[error("api client error")]
    ApiClientError(#[from] ApiClientErrors),
    #[error("index format conversion failed")]
    InvalidIndex,
    #[error("candle store for given pair is not initialized")]
    MissingCandles,
    #[error("volume store for given pair is not initialized")]
    MissingVolumes,
    #[error("provided timestamps are invalid")]
    InvalidTimestamps,
    #[error("given pair format is not supported, supported format is TOKEN-TOKEN")]
    UnsupportedPairFormat,
    #[error("could not deserialize: {message}")]
    DeserializationFailed { message: String },
    #[error("given api key not found on contract")]
    MissingApiKey,
    #[error("missing timestamp, pair/volumes are not initialized")]
    MissingTimestamp,
    #[error("given orderbook data do not exist")]
    MissingOrderbook
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

    async fn get_taker_volume(
        &self,
        pair: &Pair,
        range: std::ops::Range<u64>,
    ) -> Result<Vec<TimeVolume>, ExchangeErrors>;

    async fn get_orderbook(&self, pair: &Pair, size: u32) -> Result<OrderBook, ExchangeErrors>;
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

    async fn get_pending_orders(
        &self,
        request: GeneralOrdersListRequest,
    ) -> Result<Response, ExchangeErrors>;

    async fn get_done_orders(
        &self,
        request: GeneralOrdersListRequest,
    ) -> Result<Response, ExchangeErrors>;
}

pub trait PathFormatter {
    fn get_path(&self, template: &str) -> String;
}

impl<T> PathFormatter for T
where
    T: ApiRequest,
{
    fn get_path(&self, template: &str) -> String {
        let json_value = serde_json::to_value(self).unwrap();

        // Replace placeholders with actual values
        let mut path = template.to_string();
        if let Value::Object(map) = json_value {
            for (key, value) in map {
                if let Value::String(val) = value {
                    let placeholder = format!("{{{}}}", key);
                    path = path.replace(&placeholder, &val);
                }
            }
        }
        path
    }
}

#[cfg(test)]
mod path_formatter_test {
    use super::*;

    #[test]
    fn test_serialize_to_path() {
        #[derive(Serialize)]
        struct TestRequest {
            pub product_id: String,
            pub name: String,
        }

        impl ApiRequest for TestRequest {
            const METHOD: HttpMethod = HttpMethod::GET;
            const URI: &'static str = "/test/{product_id}/{name}";
            const HOST: &'static str = "test.com";
            const BODY: bool = false;

            type Response = ();
        }

        let request = TestRequest {
            product_id: "123".to_string(),
            name: "test".to_string(),
        };

        let path = request.get_path("/test/{product_id}/{name}");
        assert_eq!(path, "/test/123/test");
    }
}

pub trait ApiRequest: Serialize {
    const METHOD: HttpMethod;
    const URI: &'static str;
    const HOST: &'static str;
    const BODY: bool;
    const PATH_PARAMS: bool = false;

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
