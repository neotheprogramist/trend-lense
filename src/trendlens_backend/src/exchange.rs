use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{
    chain_data::{ChainData, ExchangeData},
    pair::Pair,
    remote_exchanges::{
        coinbase::Coinbase,
        okx::{api::GetInstrumentsRequest, Okx},
        OpenData,
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
        match value {
            Exchange::Okx => 0,
            Exchange::Coinbase => 1,
        }
    }
}

impl Storable for Exchange {
    const BOUND: Bound = Bound::Bounded {
        max_size: std::mem::size_of::<Exchange>() as u32,
        is_fixed_size: false,
    };

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
}

pub enum ExchangeImpl {
    Okx(Okx),
    Coinbase(Coinbase),
}

pub trait ExchangeInfo {
    fn get_pairs(&self) -> Vec<Pair>;
}

impl ExchangeInfo for Okx {
    fn get_pairs(&self) -> Vec<Pair> {
        vec![Pair::BtcUsd]
    }
}

impl ExchangeImpl {
    pub fn new(exchange_type: Exchange) -> Self {
        match exchange_type {
            Exchange::Coinbase => Self::Coinbase(Coinbase::default()),
            Exchange::Okx => Self::Okx(Okx::default()),
        }
    }

    pub fn get_pairs(&self) -> Vec<Pair> {
        match self {
            ExchangeImpl::Coinbase(_) => unimplemented!(),
            ExchangeImpl::Okx(o) => o.get_pairs(),
        }
    }

    // right now for testing purposes used single request, but it should be
    // preconstructed for further use or migrate signature generation to client
    pub fn get_signature_string(&self, request: Request) -> String {
        match self {
            ExchangeImpl::Coinbase(_) => "".to_string(),
            ExchangeImpl::Okx(o) => match request {
                Request::Instruments(i) => {
                    let request = GetInstrumentsRequest {
                        instrument_id: i.instrument_id,
                        instrument_type: i.instrument_type,
                    };
                    o.get_signature_data(request)
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

    pub fn set_data(&self, pair: Pair, data: StorableWrapper<ExchangeData>) {
        match self {
            ExchangeImpl::Coinbase(c) => c.set_data(pair, data),
            ExchangeImpl::Okx(o) => o.set_data(pair, data),
        }
    }

    pub async fn fetch_candles(
        &self,
        pair: Pair,
        range: std::ops::Range<u64>,
        interval: u32,
    ) -> Result<Vec<Candle>, super::ExchangeErrors> {
        match self {
            ExchangeImpl::Coinbase(c) => c.fetch_candles(pair, range, interval).await,
            ExchangeImpl::Okx(o) => o.fetch_candles(pair, range, interval).await,
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
}
