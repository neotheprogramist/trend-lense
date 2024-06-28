use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{
    chain_data::{ChainData, ExchangeData},
    pair::Pair,
    remote_exchanges::{coinbase::Coinbase, okx::Okx},
    storable_wrapper::StorableWrapper,
};

const MAX_EXCHANGE_SIZE: u32 = 21;

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
        max_size: MAX_EXCHANGE_SIZE,
        is_fixed_size: true,
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
        vec![]
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
            ExchangeImpl::Coinbase(_) => vec![],
            ExchangeImpl::Okx(o) => o.get_pairs(),
        }
    }

    pub fn data_mut(&self) -> StorableWrapper<ExchangeData> {
        match self {
            ExchangeImpl::Coinbase(c) => c.data_mut(),
            ExchangeImpl::Okx(o) => o.data_mut(),
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
