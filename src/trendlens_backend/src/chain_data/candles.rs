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
