use ic_cdk::api::management_canister::http_request::HttpMethod;
use serde::{Deserialize, Serialize};
use super::response;
use crate::remote_exchanges::ApiRequest;

#[derive(Deserialize, Serialize)]
pub struct GetAllPairsRequest;

impl ApiRequest for GetAllPairsRequest {
    const BODY: bool = false;
    const HOST: &'static str = "www.api.exchange.coinbase.com";
    const METHOD: HttpMethod = HttpMethod::GET;
    const URI: &'static str = "products";
    const PUBLIC: bool = true;

    type Response = Vec<response::ConcreteInstrument>;
}
