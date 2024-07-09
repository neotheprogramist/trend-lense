use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(
    CandidType, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord, Debug,
)]
pub struct Pair {
    pub base: String,
    pub quote: String,
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
