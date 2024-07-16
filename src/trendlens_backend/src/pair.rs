use std::borrow::Cow;
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

#[repr(u32)]
#[derive(CandidType, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Pair {
    BtcUsd = 0,
    EthUsd,
}

impl From<Pair> for u32 {
    fn from(value: Pair) -> Self {
        match value {
            Pair::BtcUsd => 0,
            Pair::EthUsd => 1,
        }
    }
}

impl Storable for Pair {
    const BOUND: Bound = Bound::Bounded {
        max_size: std::mem::size_of::<Pair>() as u32,
        is_fixed_size: false,
    };

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_into_u32() {
        assert_eq!(u32::from(Pair::BtcUsd), 0);
        assert_eq!(u32::from(Pair::EthUsd), 1);
    }
}
