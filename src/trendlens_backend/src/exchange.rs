use candid::CandidType;
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use core::panic;
use std::borrow::Cow;

use crate::{
    chain_data::{ChainData, ExchangeData},
    instruments::get_instruments,
    pair::Pair,
    remote_exchanges::{
        coinbase::{
            Coinbase, CoinbaseOrdersRequest, GetProfileAccountsRequest, PostOrderBody, Statuses,
        },
        okx::{
            api::{
                GetBalanceRequest, GetInstrumentsRequest, InstrumentType, OrdersHistoryRequest,
                PendingOrdersRequest as OxkPendingOrdersRequest, PlaceOrderBody,
            },
            Okx,
        },
        request::{GeneralInstrumentsRequest, OrderSide},
        response::Instrument,
        ExchangeErrors, OpenData,
    },
    request_store::request::Request,
    storable_wrapper::StorableWrapper,
};

#[repr(u8)]
#[derive(
    Deserialize, Serialize, CandidType, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy,
)]
pub enum Exchange {
    Okx,
    Coinbase,
}

impl From<Exchange> for u8 {
    fn from(value: Exchange) -> Self {
        value as u8
    }
}

impl From<u8> for Exchange {
    fn from(value: u8) -> Self {
        match value {
            0 => Exchange::Okx,
            1 => Exchange::Coinbase,
            _ => panic!("Invalid exchange type"),
        }
    }
}

impl Storable for Exchange {
    const BOUND: Bound = Bound::Bounded {
        max_size: std::mem::size_of::<Exchange>() as u32,
        is_fixed_size: true,
    };

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        bytes.as_ref()[0].into()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        ic_cdk::println!("{:?}", *self as u8);
        Cow::Owned(vec![(*self).into()])
    }
}

pub enum ExchangeImpl {
    Okx(Okx),
    Coinbase(Coinbase),
}

pub trait ExchangeId {
    fn exchange_id(&self) -> Exchange;
}

pub trait ExchangeInfo
where
    Self: ExchangeId,
{
    fn get_pairs(&self, instrument_type: InstrumentType) -> Vec<Pair> {
        get_instruments(self.exchange_id(), instrument_type)
            .unwrap_or_default()
            .into_iter()
            .map(|r| r.instrument_id)
            .collect()
    }
}

impl ExchangeId for Okx {
    fn exchange_id(&self) -> Exchange {
        Exchange::Okx
    }
}

impl<T> ExchangeInfo for T where T: ExchangeId {}

impl ExchangeImpl {
    pub fn new(exchange_type: Exchange) -> Self {
        match exchange_type {
            Exchange::Coinbase => Self::Coinbase(Coinbase::default()),
            Exchange::Okx => Self::Okx(Okx::default()),
        }
    }

    pub fn get_pairs(&self, instrument_type: InstrumentType) -> Vec<Pair> {
        match self {
            ExchangeImpl::Coinbase(c) => c.get_pairs(instrument_type),
            ExchangeImpl::Okx(o) => o.get_pairs(instrument_type),
        }
    }

    pub async fn get_market_depth(
        &self,
        pair: &Pair,
        order_side: &OrderSide,
        price_limit: u32,
    ) -> f64 {
        match self {
            ExchangeImpl::Coinbase(c) => {
                let orderbook = c
                    .get_orderbook(pair, 50)
                    .await
                    .expect("failed to get orderbook data");

                let orders = match order_side {
                    OrderSide::Buy => &orderbook.asks,
                    OrderSide::Sell => &orderbook.bids,
                };

                let start_price = orders.first().expect("no orders").price;
                let stop_price = start_price * (1.0 + price_limit as f64 / 100.0);

                orders
                    .iter()
                    .filter_map(|order| {
                        if order.price <= stop_price {
                            Some(order.size)
                        } else {
                            None
                        }
                    })
                    .fold(0.0, |acc, x| acc + x)
            }
            ExchangeImpl::Okx(o) => {
                let orderbook = o
                    .get_orderbook(pair, 50)
                    .await
                    .expect("failed to get orderbook data");

                let orders = match order_side {
                    OrderSide::Buy => &orderbook.asks,
                    OrderSide::Sell => &orderbook.bids,
                };

                let start_price = orders.first().expect("no orders").price;
                let stop_price = start_price * (1.0 + price_limit as f64 / 100.0);

                orders
                    .iter()
                    .filter_map(|order| {
                        if order.price <= stop_price {
                            Some(order.size)
                        } else {
                            None
                        }
                    })
                    .fold(0.0, |acc, x| acc + x)
            }
        }
    }

    // right now for testing purposes used single request, but it should be
    // preconstructed for further use or migrate signature generation to client
    pub fn get_signature_string(&self, request: &Request) -> String {
        match self {
            ExchangeImpl::Coinbase(c) => match request {
                Request::OrdersList(i) => {
                    let request = CoinbaseOrdersRequest {
                        product_id: Some(i.instrument_id.to_string()),
                        market_type: Some(i.instrument_type.to_string()),
                        limit: 100,
                        status: Some(match i.pending {
                            true => Statuses(vec!["open".to_string(), "pending".to_string()]),
                            false => Statuses(vec!["done".to_string(), "rejected".to_string()]),
                        }),
                    };

                    c.get_signature_data(request)
                }
                Request::Balances(_) => {
                    let request = GetProfileAccountsRequest {};

                    c.get_signature_data(request)
                }
                Request::PostOrder(request) => {
                    let exchange_request = PostOrderBody {
                        product_id: request.instrument_id.to_string(),
                        price: request.order_price,
                        side: request.side.into(),
                        funds: None,
                        size: Some(request.size.to_string()),
                        order_type: request.order_type.into(),
                    };

                    c.get_signature_data(exchange_request)
                }
                _ => "".to_string(),
            },
            ExchangeImpl::Okx(o) => match request {
                Request::OrdersList(i) => match i.pending {
                    true => {
                        let r = OxkPendingOrdersRequest {
                            instrument_id: Some(i.instrument_id.to_string()),
                            instrument_type: Some(i.instrument_type),
                        };

                        o.get_signature_data(r)
                    }
                    false => {
                        let r = OrdersHistoryRequest {
                            instrument_id: Some(i.instrument_id.to_string()),
                            instrument_type: i.instrument_type,
                        };

                        o.get_signature_data(r)
                    }
                },
                Request::Instruments(i) => {
                    let request = GetInstrumentsRequest {
                        instrument_id: i
                            .instrument_id
                            .as_ref()
                            .and_then(|p| Okx::instrument_id(&p)),
                        instrument_type: i.instrument_type,
                    };
                    o.get_signature_data(request)
                }
                Request::Balances(b) => {
                    let request = GetBalanceRequest {
                        currencies: b.currency.as_ref().and_then(|c| Some(c.join(","))),
                    };

                    o.get_signature_data(request)
                }
                Request::PostOrder(request) => {
                    let exchange_request = PlaceOrderBody {
                        side: Okx::side_string(request.side),
                        instrument_id: request.instrument_id.to_string(),
                        order_type: Okx::order_type_string(request.order_type),
                        size: request.size.to_string(),
                        order_price: request.order_price.map(|p| p.to_string()),
                        trade_mode: Okx::trade_mode_string(request.trade_mode),
                        ..Default::default()
                    };

                    o.get_signature_data(exchange_request)
                }
                _ => "".to_string(),
            },
        }
    }

    pub fn get_data(&self, pair: Pair) -> Option<StorableWrapper<ExchangeData>> {
        match self {
            ExchangeImpl::Coinbase(c) => c.get_data(pair),
            ExchangeImpl::Okx(o) => o.get_data(pair),
        }
    }

    pub async fn get_taker_volume(
        &self,
        pair: &Pair,
        range: std::ops::Range<u64>,
    ) -> Result<Vec<TimeVolume>, ExchangeErrors> {
        match self {
            ExchangeImpl::Coinbase(c) => c.get_taker_volume(pair, range).await,
            ExchangeImpl::Okx(o) => o.get_taker_volume(pair, range).await,
        }
    }

    pub fn set_data(&self, pair: Pair, data: StorableWrapper<ExchangeData>) {
        match self {
            ExchangeImpl::Coinbase(c) => c.set_data(pair, data),
            ExchangeImpl::Okx(o) => o.set_data(pair, data),
        }
    }

    pub async fn refresh_instruments(
        &self,
        instrument_type: &InstrumentType,
    ) -> Result<Vec<Instrument>, ExchangeErrors> {
        let get_instruments_request = GeneralInstrumentsRequest {
            instrument_id: None,
            instrument_type: instrument_type.clone(),
        };

        match self {
            ExchangeImpl::Coinbase(c) => c.get_public_instruments(get_instruments_request).await,
            ExchangeImpl::Okx(o) => o.get_public_instruments(get_instruments_request).await,
        }
    }

    pub async fn fetch_index_candles(
        &self,
        pair: &Pair,
        range: std::ops::Range<u64>,
        interval: u32,
    ) -> Result<Vec<Candle>, super::ExchangeErrors> {
        match self {
            ExchangeImpl::Okx(o) => o.fetch_index_candles(pair, range, interval).await,
            _ => panic!("Not implemented"),
        }
    }
}

#[derive(Deserialize, CandidType, Serialize, Clone, PartialEq, Debug)]
pub struct Candle {
    pub timestamp: u64,
    pub open_price: f64,
    pub highest_price: f64,
    pub lowest_price: f64,
    pub close_price: f64,
    pub volume: f64
}

#[derive(Deserialize, CandidType, Serialize, Clone, PartialEq, Debug)]
pub struct TimeVolume {
    pub timestamp: u64,
    pub volume: f64,
}
