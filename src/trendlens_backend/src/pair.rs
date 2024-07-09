use candid::CandidType;
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

impl Pair {
    fn decode_string(encoded: &[u16]) -> String {
        let decoded: Vec<u16> = encoded.iter().copied().take_while(|&ch| ch != 0).collect();
        String::from_utf16_lossy(&decoded)
    }

    fn encode_u16(s: &str) -> [u16; 12] {
        let mut buffer: [u16; 12] = [0; 12];
        let mut id = 0;

        for ch in s.chars() {
            let encoded_char = ch.encode_utf16(&mut buffer[id..]);
            id += encoded_char.len();
        }

        buffer
    }
}

impl Storable for Pair {
    const BOUND: Bound = Bound::Bounded {
        max_size: std::mem::size_of::<[u16; 24]>() as u32,
        is_fixed_size: true,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let base_encoded = Pair::encode_u16(&self.base);
        let quote_encoded = Pair::encode_u16(&self.quote);

        let mut result: [u16; 24] = [0; 24];

        for i in 0..base_encoded.len() {
            result[i] = base_encoded[i];
        }

        for i in 0..quote_encoded.len() {
            result[i + quote_encoded.len()] = quote_encoded[i];
        }

        Cow::Owned(bincode::serialize(&result).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let result: [u16; 24] = bincode::deserialize(bytes.as_ref()).expect("decode pair failed");

        let base = Self::decode_string(&result[0..12]);
        let quote = Self::decode_string(&result[12..24]);

        Pair { base, quote }
    }
}
