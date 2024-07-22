use crate::{
    chain_data::TimestampBased,
    exchange::{Exchange, TimeVolume},
    memory::{Memory, MemoryLocation, MEMORY_MANAGER},
    pair::Pair,
    storable_wrapper::StorableWrapper,
};
use ic_stable_structures::StableBTreeMap;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

type Timestamp = u64;
type VolumeStore = StableBTreeMap<(Exchange, Pair), StorableWrapper<VolumesStore>, Memory>;

thread_local! {
    pub static VOLUME_STORE: RefCell<VolumeStore> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryLocation::Volumes.memory_id())),
        )
    );
}

#[derive(Deserialize, Serialize, Default)]
pub struct VolumesStore(BTreeMap<Timestamp, TimeVolume>);

impl VolumesStore {
    pub fn insert_many(&mut self, candles: Vec<TimeVolume>) {
        for c in candles {
            self.insert(c.timestamp, c);
        }
    }
}

impl Deref for VolumesStore {
    type Target = BTreeMap<Timestamp, TimeVolume>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VolumesStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TimestampBased for VolumesStore {
    type Item = TimeVolume;

    fn last_timestamp(&self) -> Option<u64> {
        let (key, _) = self.last_key_value()?;
        return Some(*key);
    }

    fn get_between(&self, range: std::ops::Range<Timestamp>) -> Vec<TimeVolume> {
        self.range(range)
            .map(|(_, e)| e)
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    }
}
