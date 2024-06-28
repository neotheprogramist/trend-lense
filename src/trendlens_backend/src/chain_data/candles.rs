use super::{BTreeMap, Candle, Deserialize, Serialize, Timestamp, TimestampBased};

#[derive(Deserialize, Serialize, Default)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_candle(timestamp: u64) -> Candle {
        Candle {
            timestamp,
            open_price: 1.0,
            close_price: 1.0,
            highest_price: 1.0,
            lowest_price: 1.0,
        }
    }

    #[test]
    fn test_candles_store() {
        let mut candles = CandlesStore {
            data: BTreeMap::new(),
        };

        let candles_array = vec![create_candle(1), create_candle(2), create_candle(3)];

        candles.insert_many(candles_array.clone());

        assert_eq!(candles.last_timestamp(), Some(3));
        assert_eq!(
            candles.get_between(0..2),
            vec![candles_array.get(0).unwrap().clone()]
        );
    }
}
