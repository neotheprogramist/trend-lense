use super::{BTreeMap, Candle, Deserialize, Serialize, Timestamp, TimestampBased};

#[derive(Deserialize, Serialize)]
pub struct CandlesStore {
    data: BTreeMap<Timestamp, Candle>,
}

impl CandlesStore {
    pub fn insert_many(&mut self, candle: Vec<Candle>) {
        for c in candle {
            self.data.insert(c.timestamp, c);
        }
    }
}

impl TimestampBased for CandlesStore {
    type Item = Candle;

    fn last_timestamp(&self) -> Option<u64> {
        let (key, _) = self.data.last_key_value()?;
        return Some(*key);
    }

    fn get_between(&self, range: std::ops::Range<Timestamp>) -> Vec<Candle> {
        self.data
            .range(range)
            .map(|(_, e)| e)
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    }
}
