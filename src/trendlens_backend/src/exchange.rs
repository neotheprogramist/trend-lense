use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

const MAX_EXCHANGE_SIZE: u32 = 21;

#[repr(u8)]
#[derive(Deserialize, CandidType, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

#[derive(Deserialize, CandidType, Serialize, Clone, PartialEq, Debug)]
pub struct Candle {
    pub timestamp: u64,
    pub open_price: f64,
    pub highest_price: f64,
    pub lowest_price: f64,
    pub close_price: f64,
}
