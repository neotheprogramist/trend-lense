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

    type Response = Vec<response::Profile>;
}

#[derive(Deserialize, Serialize)]
pub struct GetAllPairsRequest;

impl ApiRequest for GetAllPairsRequest {
    const BODY: bool = false;
    const HOST: &'static str = "api-public.sandbox.exchange.coinbase.com";
    const METHOD: HttpMethod = HttpMethod::GET;
    const URI: &'static str = "products";

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
            _ => unimplemented!(),
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
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub size: Option<f64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub price: Option<f64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub funds: Option<f64>,
    #[serde(rename = "type")]
    #[serde_as(as = "DisplayFromStr")]
    pub order_type: OrderType,
    #[serde_as(as = "DisplayFromStr")]
    pub side: OrderSide,
}

#[cfg(test)]
mod post_order_test {
    use super::*;

    #[test]
    fn test_serialize_post_order_body() {
        let post_order_body = PostOrderBody {
            product_id: "BTC-USD".to_string(),
            size: Some(0.01),
            price: Some(10000.0),
            funds: None,
            order_type: OrderType::Limit,
            side: OrderSide::Buy,
        };

        let serialized = serde_qs::to_string(&post_order_body).unwrap();
        assert_eq!(
            serialized,
            "product_id=BTC-USD&size=0.01&price=10000&type=limit&side=buy"
        );

        let post_order_body = PostOrderBody {
            product_id: "BTC-USD".to_string(),
            size: None,
            price: Some(10000.0),
            funds: None,
            order_type: OrderType::Limit,
            side: OrderSide::Buy,
        };

        let serialized = serde_qs::to_string(&post_order_body).unwrap();
        assert_eq!(
            serialized,
            "product_id=BTC-USD&price=10000&type=limit&side=buy"
        );
    }
}

impl ApiRequest for PostOrderBody {
    const BODY: bool = true;
    const HOST: &'static str = "api-public.sandbox.exchange.coinbase.com";
    const METHOD: HttpMethod = HttpMethod::POST;
    const URI: &'static str = "orders";

    type Response = response::OrderResponse;
}

pub struct Statuses(pub Vec<String>);

impl Serialize for Statuses {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Statuses", 1)?;

        for status in &self.0 {
            state.serialize_field("status", status)?;
        }

        state.end()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_statuses() {
        let statuses = Statuses(vec!["open".to_string(), "pending".to_string()]);
        let serialized = serde_qs::to_string(&statuses).unwrap();

        assert_eq!(serialized, "status=open&status=pending");
    }

    #[test]
    fn test_serialize_statuses_empty() {
        let order_list_request = OrdersRequest {
            product_id: None,
            market_type: None,
            limit: 10,
            status: Some(Statuses(vec!["open".to_string(), "pending".to_string()])),
        };

        let serialized = serde_qs::to_string(&order_list_request).unwrap();
        assert_eq!(serialized, "limit=10&status=open&status=pending");
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize)]
pub struct OrdersRequest {
    pub product_id: Option<String>,
    pub market_type: Option<String>,
    pub limit: u64,
    #[serde(flatten)]
    pub status: Option<Statuses>,
}

impl ApiRequest for OrdersRequest {
    const BODY: bool = false;
    const HOST: &'static str = "api-public.sandbox.exchange.coinbase.com";
    const METHOD: HttpMethod = HttpMethod::GET;
    const URI: &'static str = "orders";

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
    const PATH_PARAMS: bool = true;

    type Response = response::OrderBook;
}

#[derive(Deserialize)]
pub struct GetProductCandles {
    pub product_id: String,
    pub granularity: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
}

impl Serialize for GetProductCandles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GetProductCandles", 4)?;

        if self.granularity.is_some() {
            state.serialize_field("granularity", &self.granularity)?;
        }

        if self.start.is_some() {
            state.serialize_field("start", &self.start)?;
        }

        if self.end.is_some() {
            state.serialize_field("end", &self.end)?;
        }

        if is_json_serializer::<S>() {
            state.serialize_field("product_id", &self.product_id)?;
        }

        state.end()
    }
}

#[cfg(test)]
mod candle_serialize_test {
    use super::*;

    #[test]
    fn test_serialize_candles() {
        let get_product_candles = GetProductCandles {
            product_id: "BTC-USD".to_string(),
            granularity: Some("60".to_string()),
            start: Some("2021-01-01T00:00:00Z".to_string()),
            end: Some("2021-01-02T00:00:00Z".to_string()),
        };

        let serialized = serde_qs::to_string(&get_product_candles).unwrap();
        assert_eq!(
            serialized,
            "granularity=60&start=2021-01-01T00%3A00%3A00Z&end=2021-01-02T00%3A00%3A00Z"
        );

        let serialized = serde_json::to_string(&get_product_candles).unwrap();

        assert_eq!(
            serialized,
            r#"{"granularity":"60","start":"2021-01-01T00:00:00Z","end":"2021-01-02T00:00:00Z","product_id":"BTC-USD"}"#
        );
    }
}

impl ApiRequest for GetProductCandles {
    const BODY: bool = false;
    const HOST: &'static str = "api.exchange.coinbase.com";
    const METHOD: HttpMethod = HttpMethod::GET;
    const URI: &'static str = "products/{product_id}/candles";
    const PATH_PARAMS: bool = true;

    type Response = Vec<response::CoinbaseCandle>;
}
