use std::{cell::RefCell, collections::BTreeMap};

use crate::{
    exchange::{Candle, Exchange},
    storable_wrapper::StorableWrapper,
};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use serde::{Deserialize, Serialize};

mod candles;
use candles::CandlesStore;

const EXCHANGE_STORE_MEMORY_ID: MemoryId = MemoryId::new(1);

type Timestamp = u64;
type Memory = VirtualMemory<DefaultMemoryImpl>;
type ExchangeStore = StableBTreeMap<Exchange, StorableWrapper<ExchangeData>, Memory>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static EXCHANGE_STORE: RefCell<ExchangeStore> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(EXCHANGE_STORE_MEMORY_ID)),
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
    const KEY: Exchange;
    type Item: for<'de> Deserialize<'de> + Serialize;

    fn init(&self);
    fn get_mut_chain_data(&self) -> StorableWrapper<Self::Item>;
    fn get_chain_data(&self) -> StorableWrapper<Self::Item>;
    fn set_chain_data(&self, data: StorableWrapper<Self::Item>);
}
