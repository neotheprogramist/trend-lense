use std::{cell::RefCell, collections::BTreeMap};

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
type ExchangeStore = StableBTreeMap<(Exchange, Pair), StorableWrapper<ExchangeData>, Memory>;

thread_local! {
    static EXCHANGE_STORE: RefCell<ExchangeStore> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryLocation::Exchanges.memory_id())),
        )
    );
}

#[derive(Deserialize, Serialize, Default)]
pub struct ExchangeData {
    pub candles: CandlesStore,
}

pub trait TimestampBased {
    type Item: for<'de> Deserialize<'de> + Serialize;

    fn last_timestamp(&self) -> Option<Timestamp>;
    fn get_between(&self, range: std::ops::Range<Timestamp>) -> Vec<Self::Item>;
}

pub trait ChainData {
    fn key(&self) -> Exchange;

    fn get_data(&self, pair: Pair) -> Option<StorableWrapper<ExchangeData>> {
        EXCHANGE_STORE.with_borrow(|b| b.get(&(self.key(), pair)))
    }

    fn set_data(&self, pair: Pair, data: StorableWrapper<ExchangeData>) {
        EXCHANGE_STORE.with_borrow_mut(|b| b.insert((self.key(), pair), data));
    }
}
