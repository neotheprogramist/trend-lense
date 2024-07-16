use ic_cdk::api::management_canister::http_request::HttpMethod;
use request::GetAllPairsRequest;
use response::CoinbaseResponse;

use crate::{
    api_client::ApiClient,
    chain_data::ChainData,
    exchange::{Candle, Exchange, ExchangeId},
};

use super::{ApiRequest, OpenData};
use response::ConcreteInstrument;

pub use request::GetProfileAccountsRequest;
pub use request::PostOrderBody;
pub use request::OrderType as CoinbaseOrderType;

mod auth;
mod request;
mod response;
mod user;

pub use auth::CoinbaseAuth;

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

impl ChainData for Coinbase {
    fn key(&self) -> Exchange {
        Exchange::Coinbase
    }
}
