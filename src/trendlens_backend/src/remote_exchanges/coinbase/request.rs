use super::response;
use crate::remote_exchanges::ApiRequest;
use ic_cdk::api::management_canister::http_request::HttpMethod;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct GetProfileAccountsRequest;

impl ApiRequest for GetProfileAccountsRequest {
    const BODY: bool = false;
    const HOST: &'static str = "api-public.sandbox.exchange.coinbase.com";
    const METHOD: HttpMethod = HttpMethod::GET;
    const URI: &'static str = "accounts";
    const PUBLIC: bool = true;

    type Response = Vec<response::Account>;
}

#[derive(Deserialize, Serialize)]
pub struct GetProfilesRequest {
    pub active: bool,
}

impl ApiRequest for GetProfilesRequest {
    const BODY: bool = false;
    const HOST: &'static str = "api-public.sandbox.exchange.coinbase.com";
    const METHOD: HttpMethod = HttpMethod::GET;
    const URI: &'static str = "profiles";
    const PUBLIC: bool = true;

    type Response = Vec<response::Profile>;
}

#[derive(Deserialize, Serialize)]
pub struct GetAllPairsRequest;

impl ApiRequest for GetAllPairsRequest {
    const BODY: bool = false;
    const HOST: &'static str = "api-public.sandbox.exchange.coinbase.com";
    const METHOD: HttpMethod = HttpMethod::GET;
    const URI: &'static str = "products";
    const PUBLIC: bool = true;

    type Response = Vec<response::ConcreteInstrument>;
}

#[derive(Deserialize, Serialize)]
pub struct PostOrderBody {
    pub product_id: String,
    pub price: Option<f64>,
    pub funds: Option<f64>,
    #[serde(rename = "type")]
    pub order_type: String,
}

impl ApiRequest for PostOrderBody {
    const BODY: bool = true;
    const HOST: &'static str = "api-public.sandbox.exchange.coinbase.com";
    const METHOD: HttpMethod = HttpMethod::POST;
    const URI: &'static str = "orders";
    const PUBLIC: bool = false;

    type Response = response::OrderResponse;
}
