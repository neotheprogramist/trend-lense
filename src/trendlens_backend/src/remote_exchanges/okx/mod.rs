use super::{Authorizable, AuthorizedData, ExchangeErrors, OpenData};
use crate::api_client::ApiClient;
use crate::chain_data::ChainData;
use crate::exchange::{Candle, Exchange};
use crate::request_store::request::Response;
use crate::Pair;
use api::{GetInstrumentsRequest, IndexCandleStickRequest, Instrument};
use ic_cdk::api::management_canister::http_request::HttpHeader;
pub mod api;
pub mod response;

pub struct OkxAuth {
    pub api_key: String,
    pub passphrase: String,
    pub signature: String,
    pub timestamp: String,
}

impl Authorizable for OkxAuth {
    fn get_auth_headers(&self) -> Vec<HttpHeader> {
        vec![
            HttpHeader {
                name: "OK-ACCESS-KEY".to_string(),
                value: self.api_key.clone(),
            },
            HttpHeader {
                name: "OK-ACCESS-SIGN".to_string(),
                value: self.signature.clone(),
            },
            HttpHeader {
                name: "OK-ACCESS-TIMESTAMP".to_string(),
                value: self.timestamp.clone(),
            },
            HttpHeader {
                name: "OK-ACCESS-PASSPHRASE".to_string(),
                value: self.passphrase.clone(),
            },
        ]
    }
}

#[derive(Default)]
pub struct Okx {
    auth: Option<OkxAuth>,
    api_client: ApiClient,
}

impl ChainData for Okx {
    fn key(&self) -> Exchange {
        Exchange::Okx
    }
}

impl Okx {
    pub fn with_auth(auth: OkxAuth) -> Self {
        Self {
            auth: Some(auth),
            ..Default::default()
        }
    }
    /// gets interval string from u32 in minutes
    fn interval_string(interval: u32) -> String {
        match interval {
            0..=1 => "1m",
            2..=3 => "3m",
            4..=u32::MAX => "5m",
        }
        .to_string()
    }

    /// gets index name from global pair enum
    fn index_name(pair: Pair) -> Option<String> {
        match pair {
            Pair::BtcUsd => Some("BTC-USD".to_string()),
            Pair::EthUsd => Some("ETH-USD".to_string()),
        }
    }
}

#[async_trait::async_trait]
impl AuthorizedData for Okx {
    async fn get_instruments(
        &self,
        req: GetInstrumentsRequest,
    ) -> Result<Response, ExchangeErrors> {
        let instrument_response = self
            .api_client
            .call(req, self.auth.as_ref())
            .await?;

        Ok(Response::Instruments(instrument_response))
    }
}

#[async_trait::async_trait]
impl OpenData for Okx {
    async fn fetch_candles(
        &self,
        pair: Pair,
        range: std::ops::Range<u64>,
        interval: u32,
    ) -> Result<Vec<Candle>, ExchangeErrors> {
        let index_name = Okx::index_name(pair).ok_or_else(|| ExchangeErrors::MissingIndex)?;

        let candle_request = IndexCandleStickRequest {
            after_timestamp: None,
            before_timestamp: Some(range.start * 1000),
            bar_size: Some(Okx::interval_string(interval)),
            index_name,
            results_limit: None,
        };

        let candle_response = self
            .api_client
            .call(candle_request, self.auth.as_ref())
            .await?;

        Ok(candle_response
            .into_iter()
            .map(|concrete_candle| concrete_candle.into())
            .collect())
    }
}

#[cfg(test)]
mod test_okx_helpers {
    use super::*;
    use crate::Pair;

    #[test]
    fn test_okx_interval_string() {
        assert_eq!(Okx::interval_string(0), "1m");
        assert_eq!(Okx::interval_string(1), "1m");
        assert_eq!(Okx::interval_string(2), "3m");
        assert_eq!(Okx::interval_string(3), "3m");
        assert_eq!(Okx::interval_string(4), "5m");
        assert_eq!(Okx::interval_string(5), "5m");
        assert_eq!(Okx::interval_string(50), "5m");
        assert_eq!(Okx::interval_string(u32::MAX), "5m");
    }

    #[test]
    fn test_okx_index_name() {
        assert_eq!(Okx::index_name(Pair::BtcUsd), Some("BTC-USD".to_string()));
        assert_eq!(Okx::index_name(Pair::EthUsd), Some("ETH-USD".to_string()));
    }
}
