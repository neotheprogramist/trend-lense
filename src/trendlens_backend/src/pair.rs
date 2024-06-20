use candid::CandidType;
use serde::Deserialize;

#[repr(u32)]
#[derive(CandidType, Clone, Copy, Deserialize)]
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
