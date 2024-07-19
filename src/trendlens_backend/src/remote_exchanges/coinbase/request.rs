use std::{fmt::Display, str::FromStr};

use super::response;
use crate::remote_exchanges::{
    request::{OrderSide, OrderType as GlobalOrderType},
    ApiRequest,
};
use ic_cdk::api::management_canister::http_request::HttpMethod;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};

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
pub enum OrderType {
    Limit,
    Market,
    Stop,
}

impl From<GlobalOrderType> for OrderType {
    fn from(value: GlobalOrderType) -> Self {
        match value {
            GlobalOrderType::Limit => OrderType::Limit,
            GlobalOrderType::Market => OrderType::Market,
            _ => unimplemented!()
        }
    }
}

impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OrderType::Limit => write!(f, "limit"),
            OrderType::Market => write!(f, "market"),
            OrderType::Stop => write!(f, "stop"),
        }
    }
}

impl FromStr for OrderType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "limit" => Ok(OrderType::Limit),
            "market" => Ok(OrderType::Market),
            "stop" => Ok(OrderType::Stop),
            _ => Err("unknown order type".to_string()),
        }
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub struct PostOrderBody {
    pub product_id: String,
    pub size: Option<String>,
    pub price: Option<f64>,
    pub funds: Option<f64>,
    #[serde(rename = "type")]
    #[serde_as(as = "DisplayFromStr")]
    pub order_type: OrderType,
    #[serde_as(as = "DisplayFromStr")]
    pub side: OrderSide,
}

impl ApiRequest for PostOrderBody {
    const BODY: bool = true;
    const HOST: &'static str = "api-public.sandbox.exchange.coinbase.com";
    const METHOD: HttpMethod = HttpMethod::POST;
    const URI: &'static str = "orders";
    const PUBLIC: bool = false;

    type Response = response::OrderResponse;
}

#[serde_as]
#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub struct PendingOrdersRequest {
    pub product_id: Option<String>,
    pub market_type: Option<String>,
}

impl ApiRequest for PendingOrdersRequest {
    const BODY: bool = false;
    const HOST: &'static str = "api-public.sandbox.exchange.coinbase.com";
    const METHOD: HttpMethod = HttpMethod::GET;
    const URI: &'static str = "orders";
    const PUBLIC: bool = false;

    type Response = Vec<response::Order>;
}

#[derive(Deserialize)]
pub struct GetOrderbookRequest {
    pub product_id: String,
    pub level: u32,
}

impl Serialize for GetOrderbookRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GetOrderbookRequest", 2)?;
        state.serialize_field("level", &self.level)?;

        if is_json_serializer::<S>() {
            state.serialize_field("product_id", &self.product_id)?;
        }

        state.end()
    }
}

fn is_json_serializer<S: Serializer>() -> bool {
    let type_name = std::any::type_name::<S>();
    type_name.contains("serde_json")
}

impl ApiRequest for GetOrderbookRequest {
    const BODY: bool = false;
    const HOST: &'static str = "api-public.sandbox.exchange.coinbase.com";
    const METHOD: HttpMethod = HttpMethod::GET;
    const URI: &'static str = "products/{product_id}/book";
    const PUBLIC: bool = true;
    const PATH_PARAMS: bool = true;

    type Response = response::OrderBook;
}
