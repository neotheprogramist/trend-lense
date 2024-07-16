use std::ops::{Deref, DerefMut};

use super::{BTreeMap, Candle, Deserialize, Serialize, Timestamp, TimestampBased};

#[derive(Deserialize, Serialize, Default)]
pub struct CandlesStore(BTreeMap<Timestamp, Candle>);

impl CandlesStore {
    pub fn insert_many(&mut self, candles: Vec<Candle>) {
        for c in candles {
            self.insert(c.timestamp, c);
        }
    }
}

impl Deref for CandlesStore {
    type Target = BTreeMap<Timestamp, Candle>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CandlesStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TimestampBased for CandlesStore {
    type Item = Candle;

    fn last_timestamp(&self) -> Option<u64> {
        let (key, _) = self.last_key_value()?;
        return Some(*key);
    }

    fn get_between(&self, range: std::ops::Range<Timestamp>) -> Vec<Candle> {
        self.range(range)
            .map(|(_, e)| e)
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn create_candle(timestamp: u64) -> Candle {
    //     Candle {
    //         timestamp,
    //         open_price: 1.0,
    //         close_price: 1.0,
    //         highest_price: 1.0,
    //         lowest_price: 1.0,
    //     }
    // }

    // #[test]
    // fn test_candles_store() {
    //     let mut candles = CandlesStore {
    //         data: BTreeMap::new(),
    //     };

    //     let candles_array = vec![create_candle(1), create_candle(2), create_candle(3)];

    //     candles.insert_many(candles_array.clone());

    //     assert_eq!(candles.last_timestamp(), Some(3));
    //     assert_eq!(
    //         candles.get_between(0..2),
    //         vec![candles_array.get(0).unwrap().clone()]
    //     );
    // }
}
