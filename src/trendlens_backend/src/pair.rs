use candid::CandidType;
use serde::{Deserialize, Serialize};

#[repr(u32)]
#[derive(CandidType, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_into_u32() {
        assert_eq!(u32::from(Pair::BtcUsd), 0);
        assert_eq!(u32::from(Pair::EthUsd), 1);
    }
}
