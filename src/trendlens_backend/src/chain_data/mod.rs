use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    ops::{Deref, DerefMut},
};

use crate::{
    exchange::{Candle, Exchange},
    memory::{Memory, MemoryLocation, MEMORY_MANAGER},
    pair::Pair,
    storable_wrapper::StorableWrapper,
};
use ic_stable_structures::StableBTreeMap;
use serde::{Deserialize, Serialize};

mod candles;
use candles::CandlesStore;

type Timestamp = u64;
type ExchangeStore = StableBTreeMap<Exchange, StorableWrapper<ExchangeData>, Memory>;

thread_local! {
    static EXCHANGE_STORE: RefCell<ExchangeStore> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryLocation::Exchanges.memory_id())),
        )
    );
}

#[derive(Deserialize, Serialize, Default)]
pub struct PairCandles(HashMap<Pair, CandlesStore>);

impl Deref for PairCandles {
    type Target = HashMap<Pair, CandlesStore>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PairCandles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PairCandles {
    pub fn insert_many(&mut self, pair: Pair, candles: Vec<Candle>) {
        if let Some(data) = self.get_mut(&pair) {
            for c in candles {
                data.insert(c.timestamp, c);
            }
        }
    }

    pub fn get_pair_candles(&self, pair: &Pair) -> Option<&CandlesStore> {
        self.get(pair)
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct ExchangeData {
    pub candles: PairCandles,
}

pub trait TimestampBased {
    type Item: for<'de> Deserialize<'de> + Serialize;

    fn last_timestamp(&self) -> Option<Timestamp>;
    fn get_between(&self, range: std::ops::Range<Timestamp>) -> Vec<Self::Item>;
}

pub trait ChainData {
    fn key(&self) -> Exchange;

    fn init(&self) {
        EXCHANGE_STORE.with_borrow_mut(|b| {
            b.insert(self.key(), StorableWrapper(ExchangeData::default()));
        });
    }

    fn data_mut(&self) -> StorableWrapper<ExchangeData> {
        EXCHANGE_STORE.with_borrow_mut(|b| b.get(&self.key()).unwrap())
    }

    fn set_data(&self, data: StorableWrapper<ExchangeData>) {
        EXCHANGE_STORE.with_borrow_mut(|b| b.insert(self.key(), data));
    }
}
